use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::use_cases::brawlers::BrawlersUseCase,
    domain::{
        repositories::brawlers::BrawlerRepository,
        value_objects::brawler_model::RegisterBrawlerModel,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = BrawlerPostgres::new(db_pool);
    let user_case = BrawlersUseCase::new(Arc::new(repository));

    Router::new()
        .route("/register", post(register))
        .with_state(Arc::new(user_case))
}

pub async fn register<T>(
    State(user_case): State<Arc<BrawlersUseCase<T>>>,
    Json(model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: BrawlerRepository + Send + Sync,
{
    match user_case.register(model).await {
        Ok(user_id) => (StatusCode::CREATED, user_id.to_string()).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
