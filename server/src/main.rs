use std::env;

use axum::{
    response::IntoResponse, routing::{get, post}, Json, Router
};

#[derive(serde::Deserialize)]
struct RatelimitRequest {
    node: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/ratelimit", post(ratelimit));

    // run our app with hyper, listening globally on port 3000
    let listener =
        tokio::net::TcpListener::bind(env::var("BIND").unwrap_or("0.0.0.0:3000".to_string()))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn ratelimit(
    headers: axum::http::HeaderMap,
    Json(body): Json<RatelimitRequest>,
)-> impl IntoResponse {
    let authorization = headers.get("Authorization").unwrap().to_str().unwrap();
    if authorization != env::var("API_KEY").unwrap() {
        return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let client = reqwest::Client::new();

    client
        .post(env::var("DISCORD_WEBHOOK_URL").unwrap())
        .json(&serde_json::json!({
            "content": env::var("DISCORD_WEBHOOK_MESSAGE").unwrap_or("Rate limit exceeded".to_string()),
            "embeds": [
                {
                    "title": "Rate limit exceeded",
                    "description": format!("Rate limit exceeded on node {}", body.node),
                    "color": 16711680
                }
            ]
        }))
        .send()
        .await
        .unwrap();

    (axum::http::StatusCode::OK, format!("Ratelimit for node {} reported", body.node)).into_response()
}
