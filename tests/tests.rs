extern crate diesel;
extern crate migrations_internals;
extern crate space;

#[cfg(test)]
mod tests {
    /*
    use std::collections::HashMap;
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};

    use diesel::pg::PgConnection;
    use diesel::prelude::*;

    use space::constants;
    use space::item::{Item, ItemType};
    use space::mass::{Mass, MassEntry};
    use space::math::{get_db_url, Vector};
    use space::modules::construction;
    use space::modules::mining;
    use space::modules::navigation;
    use space::modules::refinery;
    use space::modules::tractorbeam;
    use space::modules::types::ModuleType;
    */
    use actix_web::client::Client;
    use actix_web::http::StatusCode;

    async fn test_register() {
        let response = Client::default()
            .put("http://localhost:8000/register")
            .basic_auth("user", Some("pass"))
            .send()
            .await;
        assert!(response.unwrap().status() == StatusCode::OK);

        let response = Client::default()
            .put("http://localhost:8000/register")
            .basic_auth("user", Some("pass"))
            .send()
            .await;
        assert!(response.unwrap().status() == StatusCode::CONFLICT);
    }

    async fn test_dashboard() {
        let response = Client::default()
            .get("http://localhost:8000/play")
            .basic_auth("user", Some("pass"))
            .send_body(r#"{"module" : "dashboard"}"#)
            .await;
        assert!(response.unwrap().status() == StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_api() {
        test_register().await;
        test_dashboard().await;
    }
}
