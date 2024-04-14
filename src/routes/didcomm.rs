use crate::did::DidcommEnvelope;
use crate::key::{enc_keypair_from_seed, sign_keypair_from_seed};
use crate::server::AppState;
use crate::services::*;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OOBQuery {
    pub oob: String,
    pub id: String,
}

#[derive(Deserialize)]
pub struct IdQuery {
    pub id: String,
}

pub async fn handle_oob_request(Query(query): Query<OOBQuery>) -> impl IntoResponse {
    let data = decode_oob_data(&query.oob).unwrap();
    serde_json::to_string_pretty(&data).unwrap()
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

#[cfg(feature = "didcomm")]
pub async fn handle_didcomm_request(
    DidcommEnvelope { value }: DidcommEnvelope,
) -> impl IntoResponse {
    // Log the entire JSON payload
    tracing::info!("Received DIDComm message");

    tracing::debug!("Value: {:?}", value);

    let file = std::fs::File::create("received_didcomm.json").unwrap();
    serde_json::to_writer_pretty(file, &value).unwrap();

    //let (signing_key, _) = sign_keypair_from_seed("test123!");
    let (enc_key, _) = enc_keypair_from_seed("test123!");

    let value = handle_didcomm(value, enc_key);

    tracing::debug!("Value: {:?}", value);

    // Respond back
    "Received DIDComm message"
}
