use std::net::TcpListener;

use serde_json::json;
use sqlx::{PgConnection, Connection};

use diaspora_be::configuration::get_configuration;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = diaspora_be::startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // return the app addr to the caller
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_test() {
    // Arrange
    let address = spawn_app();

    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
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
    let address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();

    // The `Connection` trait MUST be in scope for us to invoke
    // `PgConnection::connect` - it is not an inherent method of the struct!
    let mut connection = PgConnection::connect(&connection_string).await.expect("Failed to connect to Postgres.");
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
        .post(&format!("{}/items", &address))
        .header("Content-type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT * FROM items",).fetch_one(&mut connection).await.expect("Failed to fetch saved item.");
    assert_eq!(saved.weight, Some(2.0));
    assert_eq!(saved.destination_country, Some("Canada".to_string()));
}

#[tokio::test]
async fn item_returns_422_when_required_fields_missing() {
    // Arrange
    let address = spawn_app();
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
            .post(&format!("{}/items", &address))
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
