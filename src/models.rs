use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use chrono::{self,DateTime, TimeZone, Utc};
use chrono::serde::ts_seconds::deserialize as from_ts;
use chrono::serde::ts_seconds_option;


#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct PEvent {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub datetime: DateTime<Utc>,
    #[serde(alias = "id")]
    pub eventID: i32,
    pub name: String,
    pub summary: String,
    pub url: String,
    pub r#type: String,
    pub location: Location,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Location {
    name: String,
    gps: String
}