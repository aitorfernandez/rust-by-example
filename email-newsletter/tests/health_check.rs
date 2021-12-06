use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_test() {
    // Arrange
    let address = spawn_app();
    // perform HTTP requests against our application
    let client = reqwest::Client::new();

    // Act
    let res = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[actix_rt::test]
async fn subscribe_returns_200_test() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let body = "name=foo%20bar&email=foo_bar%40gmail.com";

    // Act
    let res = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, res.status().as_u16());
}

#[actix_rt::test]
async fn subscribe_returns_400_test() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let tests = vec![
        ("name=foo%20bar", "missing the email"),
        ("email=foo_bar%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in tests {
        // Act
        let res = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");
        // Assert
        assert_eq!(
            400,
            res.status().as_u16(),
            "API fail with 400 Bad Request when the payload was {}",
            error_message,
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = email_newsletter::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
