use actix_rt;
use reqwest::header::HeaderValue;
use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::config::get_configuration;
use zero2prod::run::run;

#[actix_rt::test]
async fn health_check_succeeds() {
    let addr = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &addr))
        .send()
        .await
        .expect("Execute request");

    assert!(response.status().is_success(), "{:?}", response);
    assert_eq!(Some(0), response.content_length())
}

#[actix_rt::test]
async fn subscribe_returns_200_when_ok() {
    let addr = spawn_app();

    // HTTP stuff
    let client = reqwest::Client::new();
    let body = "name=Jean%20lasalle&email=jeannot-lasalle%40gmail.com";

    let response = client
        .post(&format!("{}/subscribe", &addr))
        .header(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .body(body)
        .send()
        .await
        .expect("Execute request");
    assert!(response.status().is_success());

    // DATABASE stuff
    let cf = get_configuration().expect("Get config");
    let mut connection = PgConnection::connect(&cf.db.connection_string())
        .await
        .expect("Connect to Pgsql");
    let subscription = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&mut connection)
        .await
        .expect("Get stored sub");

    assert_eq!("hello@gmail.com", subscription.email);
    assert_eq!("Jean", subscription.name)
}

#[actix_rt::test]
async fn subscribe_returns_400_with_missing_data() {
    let addr = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, error_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &addr))
            .header(
                reqwest::header::CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .body(body)
            .send()
            .await
            .expect("Execute request");

        assert_eq!(
            reqwest::StatusCode::BAD_REQUEST,
            response.status(),
            "When {}",
            error_msg
        );
    }
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Bind address");
    let addr = listener.local_addr().expect("Get local address");
    let server = run(listener).expect("Create the server");

    tokio::spawn(server);

    format!("http://{}:{}", addr.ip(), addr.port())
}
