use crate::server::AppState;
use crate::services::*;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};

pub async fn handle_creds_request(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let domain = state.domain.clone();
    let seed = state.seed.clone();
    let offer = create_cred_offer(id, domain, seed).unwrap();
    serde_json::to_string_pretty(&offer).unwrap()
}
