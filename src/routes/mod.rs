use crate::server::AppState;
use crate::services::*;
use axum::Router;
use axum::{
    body::Body,
    extract::{Path, State},
    http::header::CONTENT_TYPE,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};

#[cfg(feature = "didcomm")]
mod didcomm;
#[cfg(feature = "didcomm")]
pub use didcomm::*;

#[cfg(feature = "oid4vc")]
mod oid4vc;
#[cfg(feature = "oid4vc")]
pub use oid4vc::*;

pub fn router(app: AppState) -> Router {
    let route = Router::new()
        .route(
            "/",
            get(|| async { Redirect::temporary(&format!("/qr/{}", uuid::Uuid::new_v4())) }),
        )
        .route("/qr/:id", get(handle_qr_code_request))
        .route("/favicon.png", get(handle_favicon_request));

    #[cfg(feature = "didcomm")]
    let route = {
        route
            .route("/didcomm/", post(handle_didcomm_request))
            .route("/inv", get(handle_invitation_request))
            .route("/s", get(handle_invitation_request))
            .route("/ssi", get(handle_oob_request))
            .route("/.well-known/did.json", get(handle_did_doc_request))
    };

    #[cfg(feature = "oid4vc")]
    let route = { route.route("/creds/:id", get(handle_creds_request)) };

    route.with_state(app)
}

pub async fn handle_favicon_request() -> impl IntoResponse {
    let favicon = include_bytes!("../assets/favicon.png");
    Response::builder()
        .header(CONTENT_TYPE, "image/png")
        .body(Body::from(favicon.to_vec()))
        .unwrap()
}

pub async fn handle_qr_code_request(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let base_url = state.base_url.clone();
    create_qr_code(id, base_url).await.unwrap()
}
