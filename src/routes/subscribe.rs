use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribePayload {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<SubscribePayload>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions
            (id, email, name, subscribed_at)
        VALUES
            ($1,    $2,   $3,            $4)
    "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok(),
        Err(err) => {
            println!("Insert subscription: {}", err);
            HttpResponse::InternalServerError()
        }
    }
}
