use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::web::HttpResponse;
use diesel::prelude::*;
use serde::Serialize;
use std::time::SystemTime;
use uuid::Uuid;

use crate::client::types::ClientDashboard;
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
        let mut inserted = false;
        while !inserted {
            let result = diesel::insert_into(responses::dsl::responses)
                .values(&self)
                .execute(connection);
            inserted = result.is_ok();
        }
    }

    pub fn get_data(&self) -> ResponseData {
        serde_json::from_str(&self.data).unwrap()
    }

    pub fn to_http_response(&self) -> HttpResponse {
        match self.get_data().status_code.as_str() {
            "Ok" => HttpResponseBuilder::new(StatusCode::OK).finish(),
            "Conflict" => HttpResponseBuilder::new(StatusCode::CONFLICT).finish(),
            "Unauthorized" => HttpResponseBuilder::new(StatusCode::UNAUTHORIZED).finish(),
            "Not Implemented" => HttpResponseBuilder::new(StatusCode::NOT_IMPLEMENTED).finish(),
            _ => HttpResponseBuilder::new(StatusCode::IM_A_TEAPOT).finish(),
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        self.data.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData {
    status_code: String,
    reason: String,
    client_data: Option<ClientDashboard>,
}

impl ResponseData {
    pub fn new(
        status_code: &str,
        reason: &str,
        client_data: Option<ClientDashboard>,
    ) -> ResponseData {
        ResponseData {
            status_code: status_code.to_string(),
            reason: reason.to_string(),
            client_data,
        }
    }
}
