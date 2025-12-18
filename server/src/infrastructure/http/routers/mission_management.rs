use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, patch, post},
};

use crate::{
    application::use_cases::mission_management::MissionManagementUseCase,
    domain::{
        repositories::{
            mission_management::MissionManagementRepository,
            mission_viewing::MissionViewingRepository,
        },
        value_objects::mission_model::{AddMissionModel, EditMissionModel},
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                mission_management::MissionManagementPostgres,
                mission_viewing::MissionViewingPostgres,
            },
        },
        http::middlewares::auth::auth,
    },
};

pub async fn add<T1, T2>(
    State(user_case): State<Arc<MissionManagementUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Json(model): Json<AddMissionModel>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match user_case.add(user_id, model).await {
        Ok(mission_id) => (StatusCode::CREATED, mission_id.to_string()).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn edit<T1, T2>(
    State(user_case): State<Arc<MissionManagementUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Json(model): Json<EditMissionModel>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match user_case.edit(mission_id, user_id, model).await {
        Ok(mission_id) => (
            StatusCode::OK,
            format!("Edit mission_id: {} completed!!", mission_id),
        )
            .into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn remove<T1, T2>(
    State(user_case): State<Arc<MissionManagementUseCase<T1, T2>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionManagementRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
{
    match user_case.remove(mission_id, user_id).await {
        Ok(_) => (
            StatusCode::OK,
            format!("Remove mission_id: {} completed!!", mission_id),
        )
            .into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let mission_repository = MissionManagementPostgres::new(Arc::clone(&db_pool));
    let viewing_repositiory = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let user_case =
        MissionManagementUseCase::new(Arc::new(mission_repository), Arc::new(viewing_repositiory));

    Router::new()
        .route("/", post(add))
        .route("/{mission_id}", patch(edit))
        .route("/{mission_id}", delete(remove))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(user_case))
}
