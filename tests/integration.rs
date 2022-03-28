use actix_rt;
use reqwest::header::HeaderValue;
use sqlx::PgPool;
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::config::get_configuration;
use zero2prod::run::run;

pub struct TestApp {
    pub address: String,
    pub db_connection_pool: PgPool,
}

#[actix_rt::test]
async fn health_check_succeeds() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Execute request");

    assert!(response.status().is_success(), "{:?}", response);
    assert_eq!(Some(0), response.content_length())
}

#[actix_rt::test]
async fn subscribe_returns_200_when_ok() {
    let app = spawn_app().await;

    // HTTP stuff
    let client = reqwest::Client::new();
    let body = "name=Jean%20Lasalle&email=jeannot-lasalle%40gmail.com";

    let response = client
        .post(&format!("{}/subscribe", &app.address))
        .header(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        )
        .body(body)
        .send()
        .await
        .expect("Execute request");
    assert!(response.status().is_success());

    let subscription = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_connection_pool)
        .await
        .expect("Get stored sub");

    assert_eq!("jeannot-lasalle@gmail.com", subscription.email);
    assert_eq!("Jean Lasalle", subscription.name)
}

#[actix_rt::test]
async fn subscribe_returns_400_with_missing_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (body, error_msg) in test_cases {
        let response = client
            .post(&format!("{}/subscribe", &app.address))
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

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Bind address");
    let addr = listener.local_addr().expect("Get local address");

    let mut config = get_configuration().expect("Read configuration");
    config.db.name = Uuid::new_v4().to_string();
    config.db.configure().await.expect("Configure the database");

    let db_connection_pool = PgPool::connect(&config.db.connection_string())
        .await
        .expect("Connect to Postgres");

    let server = run(listener, db_connection_pool.clone()).expect("Create the server");

    tokio::spawn(server);

    TestApp {
        address: format!("http://{}:{}", addr.ip(), addr.port()),
        db_connection_pool,
    }
}
