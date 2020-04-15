use diesel::prelude::*;
use serde::Serialize;
use std::time::SystemTime;
use uuid::Uuid;

use crate::request::Request;
use crate::schema::responses;

#[derive(Debug, Serialize, Queryable, Insertable, Identifiable, Associations)]
#[belongs_to(Request)]
pub struct Response {
    id: String,
    data: String,
    time: String,
    request_id: String,
}

impl Response {
    pub fn new(data: ResponseData, request_id: String) -> Response {
        Response {
            id: Uuid::new_v4().to_string(),
            data: serde_json::to_string(&data).unwrap(),
            time: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
            request_id,
        }
    }

    pub fn insert_into(self, connection: &SqliteConnection) {
        diesel::insert_into(responses::dsl::responses)
            .values(&self)
            .execute(connection)
            .unwrap();
    }
}

#[derive(Debug, Serialize)]
pub enum ResponseData {
    Good,
    Bad,
}
