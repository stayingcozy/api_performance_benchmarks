use axum::{routing::post, Json, Router, extract::State};
use axum::response::Result;
use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use std::net::SocketAddr;
use redis::AsyncCommands;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    pg: PgPool,
    redis: redis::Client,
}

#[derive(Deserialize)]
struct Order {
    user_id: String,
    product_id: String,
    amount: f64,
    currency: String,
}

async fn handle_order(
    State(state): State<AppState>,
    Json(order): Json<Order>,
) -> Result<String, (StatusCode, String)> {
    let mut conn = state.redis.get_async_connection().await.map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Redis error".to_string()))?;
    
    let _: String = conn.get(format!("user:{}", order.user_id)).await.unwrap_or_default();
    
    let rate = match get_currency_rate(&order.currency).await {
        Ok(r) => r,
        Err(_) => 1.0,
    };
    
    sqlx::query("INSERT INTO orders (user_id, product_id, amount, currency) VALUES ($1, $2, $3, $4)")
        .bind(&order.user_id)
        .bind(&order.product_id)
        .bind(order.amount * rate)
        .bind(&order.currency)
        .execute(&state.pg)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB insert error".to_string()))?;
    
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
    
    let state = AppState { pg, redis };
    
    let app = Router::new()
        .route("/order", post(handle_order))
        .with_state(state);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    
    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}