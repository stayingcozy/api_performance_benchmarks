use axum::{routing::post, Json, Router};
use serde::Deserialize;
use sqlx::PgPool;
use std::net::SocketAddr;
use redis::AsyncCommands;
#[derive(Deserialize)]
struct Order {
    user_id: String,
    product_id: String,
    amount: f64,
    currency: String,
}
async fn handle_order(
    Json(order): Json<Order>,
    pg: axum::extract::Extension<PgPool>,
    redis: axum::extract::Extension<redis::Client>,
) -> Result<String, (axum::http::StatusCode, String)> {
    let mut conn = redis.get_async_connection().await.map_err(|_| (500.into(), "Redis error".to_string()))?;
    let _: () = conn.get::<_, String>(format!("user:{}", order.user_id)).await.unwrap_or_default();
    let rate = match get_currency_rate(&order.currency).await {
        Ok(r) => r,
        Err(_) => 1.0,
    };
    sqlx::query("INSERT INTO orders (user_id, product_id, amount, currency) VALUES ($1, $2, $3, $4)")
        .bind(&order.user_id)
        .bind(&order.product_id)
        .bind(order.amount * rate)
        .bind(&order.currency)
        .execute(&**pg)
        .await
        .map_err(|_| (500.into(), "DB insert error".to_string()))?;
    Ok("ok".into())
}
async fn get_currency_rate(currency: &str) -> Result<f64, reqwest::Error> {
    let res: serde_json::Value = reqwest::get(format!("http://external-currency-api/rate?to={}", currency))
        .await?
        .json()
        .await?;
    Ok(res["rate"].as_f64().unwrap_or(1.0))
}
#[tokio::main]
async fn main() {
    let pg = PgPool::connect(&std::env::var("POSTGRES_DSN").unwrap()).await.unwrap();
    let redis = redis::Client::open(std::env::var("REDIS_URL").unwrap()).unwrap();
    let app = Router::new()
        .route("/order", post(handle_order))
        .layer(axum::extract::Extension(pg))
        .layer(axum::extract::Extension(redis));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}