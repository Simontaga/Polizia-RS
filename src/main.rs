use diesel::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use polizia_rs::models::PEvent;
use polizia_rs::schema::Event;
use std::env;
mod schema;
mod models;
use self::schema::Event::dsl::*;
// Get JSON from API
// Json to Model
// Check if entry exists -> or
// insert entry.
// Scheduled interval.

#[tokio::main]
async fn main() {
    let mut connection = establish_connection();
    let events: Vec<PEvent> = get_api_response().await;
    update_events(&mut connection, events).await;
}

async fn update_events(connection: &mut MysqlConnection, Events: Vec<PEvent>) {
    for evt in Events {
        let event = evt.eventID.clone();
        let exists = does_event_exist(connection, evt).await;
        if exists { continue; }

        println!("Event {} does not exist", event)
    }
}


async fn does_event_exist(connection: &mut MysqlConnection, PEvent : PEvent) -> bool {
    let search = Event.filter(eventID.eq(PEvent.eventID));
    let res: Result<i64, diesel::result::Error> = search.count().get_result(connection);
    match res {
        Ok(c) => { if c > 0 { return true; } },
        Err(_) => { println!("{}", "Could not get execute query count exisiting events") }
    }

    false
}

 fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn get_api_response() -> Vec<PEvent>
{
    let response = reqwest::get("https://polisen.se/api/events")
        .await.unwrap();

    let events = response.json::<Vec<PEvent>>().await.unwrap();

    events
}