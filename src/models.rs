use serde::{Serialize, Deserialize};
use diesel::prelude::*;


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: i32,
    pub datetime: String,
    pub name: String,
    pub summary: String,
    pub url: String,
    pub r#type: String,
    pub location: Location,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub gps: String
}