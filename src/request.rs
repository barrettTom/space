use diesel::prelude::*;
use diesel::result::Error;
use serde::Serialize;
use std::thread::sleep;
use std::time::SystemTime;
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::constants;
use crate::response::Response;
use crate::schema::requests;

#[derive(Debug, Clone, Serialize, Queryable, Insertable, Identifiable)]
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

    pub fn insert_into(&self, connection: &SqliteConnection) -> Result<usize, Error> {
        diesel::insert_into(requests::dsl::requests)
            .values(self)
            .execute(connection)
    }

    pub fn mark_received(&self, connection: &SqliteConnection) {
        diesel::update(self)
            .set(requests::dsl::received.eq(true))
            .execute(connection)
            .unwrap();
    }

    pub fn get_response(&self, connection: &SqliteConnection) -> Result<Response, Error> {
        self.insert_into(connection).unwrap();
        let timer = Instant::now();
        while timer.elapsed().as_millis() < (constants::LOOP_DURATION_MS * 2).into() {
            sleep(Duration::from_millis(constants::LOOP_DURATION_MS / 6));
            if let Ok(response) = Response::belonging_to(self).first(connection) {
                return Ok(response);
            }
        }
        Response::belonging_to(self).first(connection)
    }
}

#[derive(Debug, Serialize)]
pub enum RequestData {
    Play { ship: String, module: String },
    Register { ship: String },
}
