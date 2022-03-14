use super::routes;
use actix_web::{dev::Server, web, App, HttpServer};

pub fn run(listener: std::net::TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscribe", web::post().to(routes::subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
