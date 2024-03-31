use crate::server::AppState;
use crate::services::{create_did_document, create_oob_url, create_qr_code};
use axum::Router;
use axum::{
    extract::{Path, Query, State},
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
        .route("/inv", get(handle_invitation_request))
        .route("/ssi", get(|| async { "Hello from ssi!" }))
        .route("/.well-known/did.json", get(handle_did_doc_request))
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
    Query(query): Query<IdQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let url = create_oob_url(&query.id, state.base_url.clone()).unwrap();
    Redirect::temporary(&url)
}

pub async fn handle_did_doc_request(State(state): State<AppState>) -> impl IntoResponse {
    let id = format!("did:{}", state.domain);
    let domain = state.domain.clone();
    let seed = state.seed.clone();
    create_did_document(id, domain, seed).unwrap()
}
