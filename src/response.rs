use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::web::HttpResponse;
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

    pub fn get_data(&self) -> ResponseData {
        serde_json::from_str(&self.data).unwrap()
    }

    pub fn to_http_response(&self) -> HttpResponse {
        match self.get_data() {
            ResponseData::Okay => HttpResponseBuilder::new(StatusCode::OK).finish(),
            ResponseData::Error(_reason) => HttpResponseBuilder::new(StatusCode::CONFLICT)
                .reason("TODO figure how to take reason")
                .finish(),
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        self.data.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseData {
    Okay,
    Error(String),
}
