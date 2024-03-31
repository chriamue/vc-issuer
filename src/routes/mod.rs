use crate::server::AppState;
use crate::services::{create_oob_url, create_qr_code};
use axum::Router;
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
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
        .route("/inv/:id", get(handle_invitation_request))
        .with_state(app)
}

pub async fn handle_qr_code_request(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let base_url = state.base_url.clone();
    let qr_code = create_qr_code(id, base_url).await.unwrap();
    qr_code
}

pub async fn handle_invitation_request(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let url = create_oob_url(&id, state.base_url.clone()).unwrap();
    Redirect::temporary(&url)
}
