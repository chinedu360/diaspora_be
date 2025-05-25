use std::net::TcpListener;

use diaspora_be::startup::run;
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use serde_json::json;
use sqlx::PgPool;

use diaspora_be::configuration::get_configuration;
use diaspora_be::telemetry::{get_subscriber, init_subscriber};

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(&configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to Postgres.");
    let server = run(listener, connection_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn health_check_test() {
    // Arrange
    let app = spawn_app().await;

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

#[tokio::test]
async fn item_returns_200_for_valid_json_data() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let body = json!({
        "description": "Integration test item",
        "weight": 2.0,
        "dimensions": {
            "length": 20.0,
            "width": 15.0,
            "height": 10.0
        },
        "origin_country": "Nigeria",
        "destination_country": "Canada",
        "price": 65.0
    });

    let response = client
        .post(&format!("{}/items", &app.address))
        .header("Content-type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert_eq!(201, response.status().as_u16());

    let saved = sqlx::query!("SELECT * FROM items")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved.");

    assert_eq!(saved.weight, Some(2.0));
    assert_eq!(saved.destination_country, Some("Canada".to_string()));
}

#[tokio::test]
async fn item_returns_422_when_required_fields_missing() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Generate test cases by removing required fields
    let test_cases = vec![
        (
            // Missing description
            json!({
                "weight": 2.0,
                "dimensions": {
                    "length": 20.0,
                    "width": 15.0,
                    "height": 10.0
                },
                "origin_country": "Nigeria",
                "destination_country": "Canada",
                "price": 65.0
            }),
            "missing description field",
        ),
        (
            // Missing dimensions
            json!({
                "description": "Integration test item",
                "weight": 2.0,
                "origin_country": "Nigeria",
                "destination_country": "Canada",
                "price": 65.0
            }),
            "missing dimensions field",
        ),
        (
            // Missing dimensions values
            json!({
                "description": "Integration test item",
                "weight": 2.0,
                "dimensions": {
                },
                "origin_country": "Nigeria",
                "destination_country": "Canada",
                "price": 65.0
            }),
            "dimension object is empty",
        ),
        (
            // incomplete dimensions values
            json!({
                "description": "Integration test item",
                "weight": 2.0,
                "dimensions": {
                    "length": 20.0,
                    "width": 15.0,
                },
                "origin_country": "Nigeria",
                "destination_country": "Canada",
                "price": 65.0
            }),
            "dimension object is empty",
        ),
    ];

    for (invalid_json, err_msg) in test_cases {
        //Act
        let response = client
            .post(&format!("{}/items", &app.address))
            .header("Content-type", "application/json")
            .json(&invalid_json)
            .send()
            .await
            .expect("Failed to execute request.");

        //Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return 422 unprocessable entity when the payload was {}.",
            err_msg
        );
    }
}
