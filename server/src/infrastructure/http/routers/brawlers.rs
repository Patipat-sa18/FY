use std::sync::Arc;

use axum::{
    Extension, Json, Router,
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
    application::use_cases::brawlers::BrawlersUseCase,
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_objects::{brawler_model::RegisterBrawlerModel, uploaded_img::UploadBase64Img},
    },
    infrastructure::{
        database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres},
        http::middlewares::auth::auth,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = BrawlerPostgres::new(db_pool);
    let user_case = BrawlersUseCase::new(Arc::new(repository));

    let protected_routes = Router::new()
        .route("/avatar", post(upload_avatar))
        .route("/my-missions", get(get_missions))
        .route("/stats", get(get_profile_stats))
        .route("/username", post(update_username))
        .route("/avatar-url", post(update_avatar_url))
        .route_layer(axum::middleware::from_fn(auth));

    Router::new()
        .merge(protected_routes)
        .route("/register", post(register::<BrawlerPostgres>))
        .with_state(Arc::new(user_case))
}

pub async fn get_missions<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.get_missions(brawler_id).await {
        Ok(missions) => (StatusCode::OK, Json(missions)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn register<T>(
    State(user_case): State<Arc<BrawlersUseCase<T>>>,
    jar: CookieJar,
    Json(model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match user_case.register(model).await {
        Ok(passport) => {
            let cookie = Cookie::build(("token", passport.token.clone()))
                .path("/")
                .same_site(SameSite::Lax)
                .http_only(false)
                .build();

            (jar.add(cookie), (StatusCode::CREATED, Json(passport))).into_response()
        }

        Err(e) => {
            if e.to_string().contains("Username already exists") {
                (StatusCode::CONFLICT, e.to_string()).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

pub async fn upload_avatar<T>(
    State(user_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(user_id): Extension<i32>,
    Json(model): Json<UploadBase64Img>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match user_case
        .upload_base64img(user_id, model.base64_string)
        .await
    {
        Ok(upload_img) => {
            tracing::info!("Avatar upload success for user {}: {}", user_id, upload_img.url);
            (StatusCode::OK, Json(upload_img)).into_response()
        },
        Err(e) => {
            tracing::error!("Avatar upload failed for user {}: {}", user_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        },
    }
}

pub async fn get_profile_stats<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.get_profile_stats(brawler_id).await {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn update_username<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(model): Json<crate::domain::value_objects::brawler_model::UpdateUsernameModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .update_username(brawler_id, model.new_username)
        .await
    {
        Ok(_) => (StatusCode::OK, "Username updated successfully").into_response(),
        Err(e) => {
            if e.to_string().contains("Username already exists") {
                (StatusCode::CONFLICT, e.to_string()).into_response()
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

pub async fn update_avatar_url<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Json(model): Json<crate::domain::value_objects::brawler_model::UpdateAvatarUrlModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match brawlers_use_case
        .update_avatar_url(brawler_id, model.url.clone())
        .await
    {
        Ok(_) => {
            tracing::info!("Avatar URL update success for user {}: {}", brawler_id, model.url);
            (StatusCode::OK, "Avatar URL updated successfully").into_response()
        },
        Err(e) => {
            tracing::error!("Avatar URL update failed for user {}: {}", brawler_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        },
    }
}

