use sqlx::PgPool;
use zero2prod::config::get_configuration;
use zero2prod::run::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("Read configuration");
    let address = format!("127.0.0.1:{}", config.port);
    let connection_pool = PgPool::connect(&config.db.connection_string())
        .await
        .expect("Connect to Postgres");
    let listener = std::net::TcpListener::bind(address)?;

    run(listener, connection_pool)?.await
}
