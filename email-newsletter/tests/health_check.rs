use email_newsletter::configuration::get_configuration;
use email_newsletter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
}

#[actix_rt::test]
async fn health_check_test() {
    // Arrange
    let app = spawn_app().await;
    // perform HTTP requests against our application
    let client = reqwest::Client::new();

    // Act
    let res = client
        .get(&format!("{}/health_check", &app.address))
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
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=foo%20bar&email=foo_bar%40gmail.com";

    // Act
    let res = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, res.status().as_u16());

    let saved = sqlx::query!("select email, name from subscription")
        .fetch_one(&app.pool)
        .await
        .expect("Failed to fetch saved subscriptions");

    assert_eq!(saved.email, "foo_bar@gmail.com");
    assert_eq!(saved.name, "foo bar");
}

#[actix_rt::test]
async fn subscribe_returns_400_test() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let tests = vec![
        ("name=foo%20bar", "missing the email"),
        ("email=foo_bar%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in tests {
        // Act
        let res = client
            .post(&format!("{}/subscriptions", &app.address))
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

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration");
    let pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let server = run(listener, pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp {
        address,
        pool: pool,
    }
}
