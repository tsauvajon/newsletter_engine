use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribePayload {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber", skip(form, connection),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name= %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<SubscribePayload>,
    connection: web::Data<PgPool>,
) -> impl Responder {
    tracing::info!(
        "Adding '{}' ({}) as a new subscriber.",
        form.name,
        form.email,
    );

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
        Ok(_) => {
            tracing::info!("New subscriber");
            HttpResponse::Ok()
        }
        Err(err) => {
            tracing::error!("Insert subscription: {:?}", err);
            HttpResponse::InternalServerError()
        }
    }
}
