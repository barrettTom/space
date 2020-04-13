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
}

#[derive(Debug, Serialize)]
pub enum RequestData {
    Play { ship: String, module: String },
    Register { ship: String },
}
