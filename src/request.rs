use diesel::prelude::*;
use diesel::result::Error;
use serde::Serialize;
use std::time::SystemTime;
use uuid::Uuid;

use crate::schema::requests;

#[derive(Debug, Serialize, Queryable, Insertable, Identifiable)]
pub struct Request {
    id: String,
    data: String,
    time: String,
    received: bool,
}

impl Request {
    pub fn new(data: RequestData) -> Request {
        Request {
            id: Uuid::new_v4().to_string(),
            data: serde_json::to_string(&data).unwrap(),
            time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            received: false,
        }
    }

    pub fn insert_into(self, connection: &SqliteConnection) -> Result<usize, Error> {
        diesel::insert_into(requests::dsl::requests)
            .values(&self)
            .execute(connection)
    }

    pub fn mark_received(&self, connection: &SqliteConnection) {
        diesel::update(self)
            .set(requests::dsl::received.eq(true))
            .execute(connection)
            .unwrap();
    }
}

#[derive(Debug, Serialize)]
pub enum RequestData {
    Play { ship: String, module: String },
    Register { ship: String },
}
