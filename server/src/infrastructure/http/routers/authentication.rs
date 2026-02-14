use std::sync::Arc;
use axum::{
    Json, Router,
    extract::{State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use crate::{
    application::use_cases::authentication::AuthenticationUseCase,
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres},
    },
};

pub async fn login<T>(
    State(user_case): State<Arc<AuthenticationUseCase<T>>>,
    jar: CookieJar,
    Json(model): Json<crate::infrastructure::jwt::authentication_model::LoginModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match user_case.login(model).await {
        Ok(passport) => {
            let cookie = Cookie::build(("token", passport.token.clone()))
                .path("/")
                .same_site(SameSite::Lax)
                .http_only(false) // Let JS read it for syncing to localStorage
                .build();

            (jar.add(cookie), Json(passport)).into_response()
        }

        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn get_me<T>(
    State(user_case): State<Arc<AuthenticationUseCase<T>>>,
    jar: CookieJar,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    if let Some(token_cookie) = jar.get("token") {
        let token = token_cookie.value();
        if let Ok(claims) = crate::infrastructure::jwt::verify_token(crate::config::config_loader::get_jwt_env().unwrap().secret, token.to_string()) {
            if let Ok(user_id) = claims.sub.parse::<i32>() {
                if let Ok(passport) = user_case.get_me(user_id).await {
                    return (StatusCode::OK, Json(passport)).into_response();
                }
            }
        }
    }
    
    (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = BrawlerPostgres::new(db_pool);
    let user_case = AuthenticationUseCase::new(Arc::new(repository));

    Router::new()
        .route("/login", post(login::<BrawlerPostgres>))
        .route("/me", get(get_me::<BrawlerPostgres>))
        .with_state(Arc::new(user_case))
}
