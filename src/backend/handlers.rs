use crate::backend::{
    models::{EvaluationTest, TestSubmission, UpdateQuestionRequest},
    service::EvaluationService,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use qrcode::{render::svg, QrCode};
use std::collections::HashMap;
use std::sync::Arc;

pub struct AppState {
    pub service: Arc<EvaluationService>,
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/api/tests",
            post(upload_test_handler).get(list_tests_handler).delete(delete_tests_handler),
        )
        .route("/api/tests/all", delete(delete_all_tests_handler))
        .route("/api/tests/:test_id", get(get_test_handler))
        .route(
            "/api/tests/:test_id/questions/:question_id",
            put(update_question_handler),
        )
        .route("/api/departments", get(get_departments_handler))
        .route("/api/participant-sections", get(get_participant_sections_handler))
        .route("/api/participants", get(get_participants_handler))
        .route("/api/access-info", get(get_access_info_handler))
        .route("/api/submit", post(submit_test_handler))
        .route("/api/results", get(get_results_handler).delete(delete_results_handler))
        .route("/api/results/all", delete(delete_all_results_handler))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(50 * 1024 * 1024)) // 50MB limit
        .with_state(state)
}

async fn get_access_info_handler() -> impl IntoResponse {
    const PORT: u16 = 5113;

    let addresses = std::process::Command::new("ifconfig")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| {
            output
                .lines()
                .filter_map(|line| {
                    let fields: Vec<&str> = line.split_whitespace().collect();
                    fields.windows(2).find_map(|pair| {
                        (pair[0] == "inet").then(|| pair[1].trim_start_matches("addr:").to_owned())
                    })
                })
                .filter(|address| address != "127.0.0.1")
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let access_points = addresses
        .iter()
        .map(|address| {
            let url = format!("http://{address}:{PORT}");
            let qr_svg = QrCode::new(url.as_bytes()).ok().map(|code| {
                code.render::<svg::Color>()
                    .min_dimensions(220, 220)
                    .build()
            });
            serde_json::json!({ "url": url, "qr_svg": qr_svg })
        })
        .collect::<Vec<_>>();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "port": PORT,
            "addresses": addresses,
            "access_points": access_points,
        })),
    )
}

async fn get_departments_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.service.get_departments().await {
        Ok(departments) => (StatusCode::OK, Json(departments)).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error})),
        )
            .into_response(),
    }
}

async fn get_participant_sections_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.service.get_participant_sections().await {
        Ok(sections) => (StatusCode::OK, Json(sections)).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error})),
        )
            .into_response(),
    }
}

async fn get_participants_handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let section = params.get("section").cloned().unwrap_or_default();
    match state.service.get_participants_by_section(&section).await {
        Ok(names) => (StatusCode::OK, Json(names)).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error})),
        )
            .into_response(),
    }
}

async fn upload_test_handler(
    State(state): State<Arc<AppState>>,
    Json(test): Json<EvaluationTest>,
) -> impl IntoResponse {
    match state.service.upload_test(test).await {
        Ok(test_id) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "success",
                "message": "Test uploaded successfully",
                "test_id": test_id,
            })),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"status": "error", "message": e})),
        ),
    }
}

async fn list_tests_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let test_ids = state.service.list_tests().await;
    (StatusCode::OK, Json(test_ids))
}

async fn get_test_handler(
    State(state): State<Arc<AppState>>,
    Path(test_id): Path<String>,
) -> impl IntoResponse {
    match state.service.get_test(&test_id).await {
        Some(test) => (StatusCode::OK, Json(serde_json::json!(test))).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Test not found"})),
        )
            .into_response(),
    }
}

async fn update_question_handler(
    State(state): State<Arc<AppState>>,
    Path((test_id, question_id)): Path<(String, String)>,
    Json(body): Json<UpdateQuestionRequest>,
) -> impl IntoResponse {
    match state
        .service
        .update_question(&test_id, &question_id, body.question_text, body.image)
        .await
    {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}

async fn submit_test_handler(
    State(state): State<Arc<AppState>>,
    Json(submission): Json<TestSubmission>,
) -> impl IntoResponse {
    match state.service.evaluate_and_save(submission).await {
        Ok(result) => (StatusCode::OK, Json(serde_json::json!(result))).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}

async fn delete_results_handler(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<i64>>,
) -> impl IntoResponse {
    match state.service.delete_results(ids).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error})),
        )
            .into_response(),
    }
}

async fn delete_tests_handler(
    State(state): State<Arc<AppState>>,
    Json(ids): Json<Vec<String>>,
) -> impl IntoResponse {
    match state.service.delete_tests(ids).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

async fn delete_all_tests_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.service.delete_all_tests().await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

async fn delete_all_results_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.service.delete_all_results().await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"status": "success"}))).into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": error})),
        )
            .into_response(),
    }
}

async fn get_results_handler(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.service.get_results().await {
        Ok(results) => (StatusCode::OK, Json(results)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}
