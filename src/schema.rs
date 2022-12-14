diesel::table! {
    Event (id) {
        id -> Integer,
        eventID -> Integer,
        datetime -> Datetime,
        name -> Varchar,
        summary -> Varchar,
        url -> Varchar,
        #[sql_name = "type"]
        type_ -> Varchar,
        locationName -> Varchar,
        locationGps -> Varchar,
    }
}