use serde::{Deserialize, Serialize};
use x25519_dalek::PublicKey;

#[derive(Serialize, Deserialize, Debug)]
pub struct DidCommInvitation {
    #[serde(rename = "@type")]
    type_: String,
    #[serde(rename = "@id")]
    id: String,
    label: String,
    #[serde(rename = "serviceEndpoint")]
    service_endpoint: String,
    #[serde(rename = "recipientKeys")]
    recipient_keys: Vec<String>,
    #[serde(rename = "routingKeys")]
    routing_keys: Vec<String>,
    did: String,
}

impl DidCommInvitation {
    pub fn new(id: String, base_url: String, label: Option<String>, enc_key: PublicKey) -> Self {
        let base_58_key = bs58::encode(enc_key.to_bytes()).into_string();

        let service_endpoint = format!("{}/didcomm", base_url);
        let type_ = "https://didcomm.org/connections/1.0/invitation".to_string();
        let label = label.unwrap_or("Invitation to connect".to_string());
        let recipient_keys = vec![base_58_key.clone()];
        let routing_keys = vec![base_58_key];
        let did = id.to_string();

        DidCommInvitation {
            type_,
            id,
            label,
            service_endpoint,
            recipient_keys,
            routing_keys,
            did,
        }
    }
}
