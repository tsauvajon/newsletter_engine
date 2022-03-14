use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscribePayload {
    name: String,
    email: String,
}

pub async fn subscribe(_form: web::Form<SubscribePayload>) -> impl Responder {
    HttpResponse::Ok()
}
