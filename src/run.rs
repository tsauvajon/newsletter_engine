use super::routes;
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;

pub fn run(
    listener: std::net::TcpListener,
    db_connection_pool: PgPool,
) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(db_connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscribe", web::post().to(routes::subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
