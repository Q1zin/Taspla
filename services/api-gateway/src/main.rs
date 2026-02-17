use axum::{
    body::Body,
    extract::{Request, State},
    http::{HeaderMap, StatusCode, Uri},
    response::Response,
    Router,
    routing::any,
};
use dotenvy::dotenv;
use reqwest::Client;
use std::env;

#[derive(Clone)]
struct AppState {
    client: Client,
    auth_service_url: String,
    tasks_service_url: String,
}

async fn proxy(
    State(state): State<AppState>,
    req: Request,
) -> Result<Response, (StatusCode, String)> {
    let path = req.uri().path();
    let query = req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();

    // Роутинг: определяем куда идёт запрос
    let target_url = if path.starts_with("/api/auth") || path.starts_with("/api/users") {
        let stripped = path.strip_prefix("/api").unwrap_or(path);
        format!("{}{}{}", state.auth_service_url, stripped, query)
    } else if path.starts_with("/api/tasks") || path.starts_with("/api/settings") {
        let stripped = path.strip_prefix("/api").unwrap_or(path);
        format!("{}{}{}", state.tasks_service_url, stripped, query)
    } else {
        return Err((StatusCode::NOT_FOUND, "Unknown route".to_string()));
    };

    // Пробрасываем метод, заголовки и тело
    let method = reqwest::Method::from_bytes(req.method().as_str().as_bytes())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let headers = req.headers().clone();
    let body_bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let mut request_builder = state.client.request(method, &target_url);

    // Копируем заголовки
    for (key, value) in headers.iter() {
        if key != "host" {
            request_builder = request_builder.header(key, value);
        }
    }

    let response = request_builder
        .body(body_bytes)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    // Пробрасываем ответ обратно
    let status = axum::http::StatusCode::from_u16(response.status().as_u16())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    let mut response_headers = HeaderMap::new();
    for (key, value) in response.headers().iter() {
        if let Ok(k) = axum::http::HeaderName::from_bytes(key.as_str().as_bytes()) {
            response_headers.insert(k, value.clone());
        }
    }

    let body_bytes = response.bytes().await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    let mut axum_response = Response::new(Body::from(body_bytes));
    *axum_response.status_mut() = status;
    *axum_response.headers_mut() = response_headers;

    Ok(axum_response)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = AppState {
        client: Client::new(),
        auth_service_url: env::var("AUTH_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:3001".to_string()),
        tasks_service_url: env::var("TASKS_SERVICE_URL")
            .unwrap_or_else(|_| "http://localhost:3002".to_string()),
    };

    let app = Router::new()
        .route("/api/*path", any(proxy))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("API Gateway running on port 8080");
    axum::serve(listener, app).await.unwrap();
}
