use actix_web::web::HttpResponse;
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

    pub fn get_data(&self) -> RequestData {
        serde_json::from_str(&self.data).unwrap()
    }

    pub fn insert_into(&self, connection: &SqliteConnection) {
        let mut inserted = false;
        while !inserted {
            let result = diesel::insert_into(requests::dsl::requests)
                .values(self)
                .execute(connection);
            inserted = result.is_ok();
        }
    }

    pub fn mark_received(&self, connection: &SqliteConnection) {
        diesel::update(self)
            .set(requests::dsl::received.eq(true))
            .execute(connection)
            .unwrap();
    }

    fn get_response(&self, connection: &SqliteConnection) -> Result<Response, Error> {
        self.insert_into(connection);
        let timer = Instant::now();
        while timer.elapsed().as_millis() < (constants::LOOP_DURATION_MS * 2).into() {
            sleep(Duration::from_millis(constants::LOOP_DURATION_MS / 6));
            if let Ok(response) = Response::belonging_to(self).first(connection) {
                return Ok(response);
            }
        }
        Response::belonging_to(self).first(connection)
    }

    pub fn get_http_response(&self, connection: &SqliteConnection) -> HttpResponse {
        match self.get_response(connection) {
            Ok(response) => response.to_http_response(),
            Err(_) => HttpResponse::RequestTimeout().finish(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestData {
    Play {
        user: String,
        pass: String,
        module: String,
    },
    Register {
        user: String,
        pass: String,
    },
}
