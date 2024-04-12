use crate::did::DidcommEnvelope;
use crate::key::{enc_keypair_from_seed, sign_keypair_from_seed};
use crate::server::AppState;
use crate::services::{
    create_did_document, create_oob_url, create_qr_code, decode_oob_data, handle_didcomm,
};
use axum::Router;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::header::CONTENT_TYPE,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: String,
}

#[derive(Deserialize)]
pub struct OOBQuery {
    pub oob: String,
    pub id: String,
}

pub fn router(app: AppState) -> Router {
    Router::new()
        .route("/qr/:id", get(handle_qr_code_request))
        .route("/inv", get(handle_invitation_request))
        .route("/ssi", get(handle_oob_request))
        .route("/didcomm", post(handle_didcomm_request))
        .route("/.well-known/did.json", get(handle_did_doc_request))
        .route("/favicon.png", get(handle_favicon_request))
        .with_state(app)
}

pub async fn handle_favicon_request() -> impl IntoResponse {
    let favicon = include_bytes!("../assets/favicon.png");
    Response::builder()
        .header(CONTENT_TYPE, "image/png")
        .body(Body::from(favicon.to_vec()))
        .unwrap()
}

pub async fn handle_oob_request(Query(query): Query<OOBQuery>) -> impl IntoResponse {
    let data = decode_oob_data(&query.oob).unwrap();
    serde_json::to_string_pretty(&data).unwrap()
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
    let seed = state.seed.clone();
    let url = create_oob_url(
        &query.id,
        state.base_url.clone(),
        state.domain.clone(),
        seed,
    )
    .unwrap();
    Redirect::temporary(&url)
}

pub async fn handle_did_doc_request(State(state): State<AppState>) -> impl IntoResponse {
    let id = format!("did:{}", state.domain);
    let domain = state.domain.clone();
    let seed = state.seed.clone();
    create_did_document(id, domain, seed).unwrap()
}

pub async fn handle_didcomm_request(
    DidcommEnvelope { value }: DidcommEnvelope,
) -> impl IntoResponse {
    // Log the entire JSON payload
    tracing::info!("Received DIDComm message");

    tracing::debug!("Value: {:?}", value);

    //let (signing_key, _) = sign_keypair_from_seed("test123!");
    let (enc_key, _) = enc_keypair_from_seed("test123!");

    let value = handle_didcomm(value, enc_key);

    // save to received.json
    //

    let file = std::fs::File::create("received.json").unwrap();
    serde_json::to_writer_pretty(file, &value.as_ref().unwrap()).unwrap();

    tracing::debug!("Value: {:?}", value);

    // Respond back
    "Received DIDComm message"
}
