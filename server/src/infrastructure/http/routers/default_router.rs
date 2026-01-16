use axum::{Router, extract::Path, http::StatusCode, response::IntoResponse, routing::get};

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, " All Right, I'am Good").into_response()
}

pub async fn make_error(Path(code): Path<u16>) -> impl IntoResponse {
    // pub async fn make_error() -> impl IntoResponse {
    // let code = 401;
    let status_code = StatusCode::from_u16(code).unwrap();
    (status_code, code.to_string()).into_response()
}

pub fn routes() -> Router {
    Router::new()
        .route("/make-error/{code}", get(make_error))
        .route("/health-check", get(health_check))
}
