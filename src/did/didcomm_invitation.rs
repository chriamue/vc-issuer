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
    #[serde(rename = "imageUrl")]
    image_url: Option<String>,
    #[serde(rename = "recipientKeys")]
    recipient_keys: Vec<String>,
    #[serde(rename = "routingKeys")]
    routing_keys: Vec<String>,
    did: String,
}

impl DidCommInvitation {
    pub fn new(id: String, base_url: String, label: Option<String>, enc_key: PublicKey) -> Self {
        let base_58_key = bs58::encode(enc_key.to_bytes()).into_string();

        let domain_parts: Vec<&str> = base_url.split('.').collect();

        let service_endpoint = format!("{}/didcomm/", base_url);
        let type_ = "https://didcomm.org/connections/1.0/invitation".to_string();
        let label = label.unwrap_or(domain_parts[1..].join("."));
        let recipient_keys = vec![base_58_key.clone()];
        let routing_keys = vec![base_58_key];
        let did = format!("did:web:{}", domain_parts[1..].join(":"));
        let image_url = Some(format!("{}/favicon.png", base_url));

        DidCommInvitation {
            type_,
            id,
            label,
            service_endpoint,
            image_url,
            recipient_keys,
            routing_keys,
            did,
        }
    }
}
