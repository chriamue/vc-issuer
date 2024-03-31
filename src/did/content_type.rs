use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Request},
};
use serde_json::Value;
use std::convert::Infallible;

#[derive(Debug)]
pub struct DidcommEnvelope {
    pub value: Value,
}

#[async_trait]
impl<B: Send + Sync> FromRequest<B> for DidcommEnvelope {
    type Rejection = Infallible;

    async fn from_request(req: Request, _state: &B) -> Result<Self, Self::Rejection> {
        let body = Bytes::from_request(req, &mut ()).await.unwrap();
        let value = serde_json::from_slice(&body).unwrap();
        Ok(DidcommEnvelope { value })
    }
}
