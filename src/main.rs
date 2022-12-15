use diesel::MysqlConnection;
use diesel::insert_into;
use diesel::prelude::*;
use dotenvy::dotenv;
use polizia_rs::models::APIEvent;
use std::env;
use std::{thread, time};
mod schema;
mod models;
use self::schema::Event::dsl::*;
use std::thread::sleep;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let interval_env = env::var("SCAN_INTERVAL_SECONDS").expect("Missing SCAN_INTERVAL_SECONDS Env Var");
    let interval = time::Duration::from_secs(interval_env.parse::<u64>().unwrap());

    let mut connection = establish_connection();

    loop {
        let events: Vec<APIEvent> = get_api_response().await;
        update_events(&mut connection, events).await;
        thread::sleep(interval);
    }
}

async fn update_events(connection: &mut MysqlConnection, Events: Vec<APIEvent>) {
    for i in 0..Events.len() {
        println!("Progress: {}/{}", i+1, Events.len());
        let event = Events[i].clone();
        let exists = does_event_exist(connection, event.clone()).await;
        if exists { continue; }
        insert_new_event(connection, event)
    }
}

fn insert_new_event(connection: &mut MysqlConnection, APIEvent: APIEvent) {
    let dbevent = models::DBEvent {
        eventID: APIEvent.eventID,
        datetime: APIEvent.datetime.naive_local(),
        locationGps: APIEvent.location.gps,
        locationName: APIEvent.location.name,
        summary: APIEvent.summary,
        url: APIEvent.url,
        name: APIEvent.name,
        type_: APIEvent.r#type,
    };

    insert_into(Event).values(&dbevent).execute(connection);
}

async fn does_event_exist(connection: &mut MysqlConnection, APIEvent : APIEvent) -> bool {
    let search = Event.filter(eventID.eq(APIEvent.eventID));
    let res: Result<i64, diesel::result::Error> = search.count().get_result(connection);
    match res {
        Ok(c) => { if c > 0 { return true; } },
        Err(_) => { println!("{}", "Could not get execute query count exisiting events") }
    }

    false
}

 fn establish_connection() -> MysqlConnection {

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn get_api_response() -> Vec<APIEvent>
{
    let response = reqwest::get("http://polisen.se/api/events")
        .await.unwrap();

    let events = response.json::<Vec<APIEvent>>().await.unwrap();

    events
}