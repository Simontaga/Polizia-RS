use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use chrono::{self,DateTime, Utc, NaiveDateTime};

use crate::schema::Event;


#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct APIEvent {
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
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Location {
   pub name: String,
   pub gps: String
}

#[derive(diesel::Insertable)]
#[table_name = "Event"]
pub struct DBEvent {
    pub datetime: NaiveDateTime,
    pub eventID: i32,
    pub name: String,
    pub summary: String,
    pub url: String,
    pub type_: String,
    pub locationName: String,
    pub locationGps: String
}