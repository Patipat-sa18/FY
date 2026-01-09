use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};

use crate::{
    application::use_cases::mission_viewing::MissionViewingUseCase,
    domain::{
        repositories::mission_viewing::MissionViewingRepository,
        value_objects::mission_filter::MissionFilter,
    },
    infrastructure::database::{
        postgresql_connection::PgPoolSquad, repositories::mission_viewing::MissionViewingPostgres,
    },
};

pub async fn get_one<T>(
    State(user_case): State<Arc<MissionViewingUseCase<T>>>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match user_case.get_one(mission_id).await {
        Ok(model) => (StatusCode::OK, Json(model)).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_crew<T>(
    State(user_case): State<Arc<MissionViewingUseCase<T>>>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match user_case.get_crew(mission_id).await {
        Ok(model) => (StatusCode::OK, Json(model)).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_all<T>(
    State(user_case): State<Arc<MissionViewingUseCase<T>>>,
    filter: Query<MissionFilter>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match user_case.get_all(&filter).await {
        Ok(model) => (StatusCode::OK, Json(model)).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let viewing_repositiory = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let user_case = MissionViewingUseCase::new(Arc::new(viewing_repositiory));

    Router::new()
        .route("/{mission_id}", get(get_one))
        .route("/filter", get(get_all))
        .route("/crew/{mission_id}", get(get_crew))
        // .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(user_case))
}
