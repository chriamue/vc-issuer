use anyhow::Result;
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PublicKey {
    id: String,
    #[serde(rename = "type")]
    key_type: String,
    controller: String,
    #[serde(rename = "publicKeyHex")]
    public_key_hex: String,
}

#[derive(Serialize, Deserialize)]
pub struct Service {
    id: String,
    #[serde(rename = "type")]
    service_type: String,
    #[serde(rename = "serviceEndpoint")]
    service_endpoint: String,
}

#[derive(Serialize, Deserialize)]
pub struct DIDDocument {
    #[serde(rename = "@context")]
    context: String,
    id: String,
    #[serde(rename = "publicKey")]
    public_key: Vec<PublicKey>,
    service: Vec<Service>,
}

impl DIDDocument {
    pub fn new(id: String, verify_key: VerifyingKey, domain: String) -> Self {
        Self {
            context: "https://www.w3.org/ns/did/v1".to_string(),
            id: format!("did:web:{}", domain),
            public_key: vec![PublicKey {
                id: format!("did:web:{}#owner", domain),
                key_type: "Ed25519VerificationKey2018".to_string(),
                controller: format!("did:web:{}", domain),
                public_key_hex: hex::encode(verify_key.as_bytes()),
            }],
            service: vec![Service {
                id: format!("did:web:{}#vcs", domain),
                service_type: "VerifiableCredentialService".to_string(),
                service_endpoint: format!("https://{}/vc/", domain),
            }],
        }
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self)?)
    }
}
