use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::{config::config_loader::get_user_secret, infrastructure::jwt::verify_token};

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_string();

    let secret = get_user_secret().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(secret, token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
