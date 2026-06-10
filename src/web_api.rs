use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::models::{ReloadedData, TaskModel};
use crate::service::TaskService;

pub fn router(service: Arc<TaskService>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/tasks", get(list_tasks).post(create_task))
        .route("/api/tasks/{id}", delete(remove_task))
        .route("/api/tasks/{id}/move", put(update_task_status))
        .route("/api/logs", get(get_logs))
        .route("/api/admin/tenants", get(list_admin_tenants))
        .fallback_service(
            tower_http::services::ServeDir::new("web/dist").append_index_html_on_directories(true),
        )
        .layer(cors)
        .with_state(service)
}

#[derive(Deserialize)]
pub struct ListTasksQuery {
    pub search: Option<String>,
}

pub async fn list_admin_tenants(
    State(service): State<Arc<TaskService>>,
) -> Result<Json<Vec<serde_json::Value>>, StatusCode> {
    match service.get_admin_tenants().await {
        Ok(data) => Ok(Json(data)),
        Err(e) => {
            eprintln!("Error getting admin tenants: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_tasks(
    State(service): State<Arc<TaskService>>,
    headers: axum::http::HeaderMap,
    Query(query): Query<ListTasksQuery>,
) -> Result<Json<ReloadedData>, (StatusCode, String)> {
    let session_id = headers
        .get("x-session-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("default-session");

    match service.reload_data(session_id, &query.search).await {
        Ok(data) => Ok(Json(data)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct CreateTaskReq {
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateTaskRes {
    pub id: u64,
}

#[axum::debug_handler]
pub async fn create_task(
    State(service): State<Arc<TaskService>>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<CreateTaskReq>,
) -> impl axum::response::IntoResponse {
    let session_id = headers
        .get("x-session-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("default-session");

    match service.add_task(session_id, &payload.name).await {
        Ok(id) => Json(CreateTaskRes { id }).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn remove_task(
    State(service): State<Arc<TaskService>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<u64>,
) -> impl axum::response::IntoResponse {
    let session_id = headers
        .get("x-session-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("default-session");

    match service.delete_task(session_id, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Deserialize)]
pub struct MoveTaskReq {
    pub status: String,
}

pub async fn update_task_status(
    State(service): State<Arc<TaskService>>,
    headers: axum::http::HeaderMap,
    Path(id): Path<u64>,
    Json(payload): Json<MoveTaskReq>,
) -> impl axum::response::IntoResponse {
    let session_id = headers
        .get("x-session-id")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("default-session");

    match service.move_task(session_id, id, &payload.status).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_logs(
    State(service): State<Arc<TaskService>>,
) -> impl axum::response::IntoResponse {
    let logs = service.check_sql_logs();
    Json(logs).into_response()
}
