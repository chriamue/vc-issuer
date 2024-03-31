use crate::server::AppState;
use crate::services::create_qr_code;
use axum::Router;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: String,
}

pub fn router(app: AppState) -> Router {
    Router::new()
        .route("/qr/:id", get(handle_qr_code_request))
        .with_state(app)
}

async fn handle_qr_code_request(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let base_url = state.base_url.clone();
    let qr_code = create_qr_code(id, base_url).await.unwrap();
    qr_code
}
