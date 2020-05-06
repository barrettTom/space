extern crate space;

#[cfg(test)]
mod tests {
    use actix_web::client::Client;
    use actix_web::http::StatusCode;
    use space::client::types::ClientDashboard;

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

    async fn test_password() {
        let response = Client::default()
            .get("http://localhost:8000/play")
            .basic_auth("user", Some("pass2"))
            .send_body(r#"{"module" : "dashboard"}"#)
            .await;
        assert!(response.unwrap().status() == StatusCode::UNAUTHORIZED);
    }

    async fn test_login() {
        let response = Client::default()
            .get("http://localhost:8000/play")
            .basic_auth("user", Some("pass"))
            .send_body(r#"{"module" : "dashboard"}"#)
            .await;
        assert!(response.unwrap().status() == StatusCode::OK);
    }

    async fn test_dashboard() {
        let response = Client::default()
            .get("http://localhost:8000/play")
            .basic_auth("user", Some("pass"))
            .send_body(r#"{"module" : "dashboard"}"#)
            .await;
        let d: Result<ClientDashboard, _> = response.unwrap().json().await;
        assert!(d.is_ok());
    }

    #[actix_rt::test]
    async fn test_api() {
        test_register().await;
        test_login().await;
        test_password().await;
        test_dashboard().await;
    }
}
