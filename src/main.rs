use diesel::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use polizia_rs::models::Event;
use std::env;

mod models;

// Get JSON from API
// Json to Model
// Check if entry exists -> or
// insert entry.
// Scheduled interval.

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let connection = establish_connection();
    let events: Vec<Event> = get_api_response().await;

    println!("{}", events.len());
}

async fn update_events(connection: MysqlConnection, Events: Vec<Event>) {
    for e in 0..Events.len() {
        
    }
}

async fn does_event_exist(connection: MysqlConnection, event : Event) -> bool {
    let existing = Event::
}

 fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn get_api_response() -> Vec<Event>
{
    let response = reqwest::get("https://polisen.se/api/events")
        .await.unwrap();

    let events = response.json::<Vec<Event>>().await.unwrap();

    events
}