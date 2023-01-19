use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]

pub async fn subscribe(
    form: web::Form<FormData>,
    // Retrieving a connection from the application state
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // let request_id = Uuid::new_v4();
    // Spans, like logs, have an associated level
    // `info_span` creates a span at the info-level
    // let request_span = tracing::info_span!("Saving new subscriber details in the database",);

    // Using `enter` in an async function is a recipe for disaster!
    // Bear with me for now, but don't fo this at home.
    // See the following section on `Instrumenting Futures`
    // let _request_span_guard = request_span.enter();
    // `_request_span_guard` is dropped at the end of `subscribe`
    // That's when we "exit" the span

    // We do not call `.enter` on query_span!
    // `.instrument` takes care of it at the right moments
    // in the query future lifetime

    // let query_span = tracing::info_span!("Saving new subscriber details in the database");

    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]

pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        sqlx::types::Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}
