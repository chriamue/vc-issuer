use anyhow::Result;
use base64::engine::general_purpose;
use base64::Engine as _;
use didcomm_rs::Message;
use ed25519_dalek::SigningKey;
use qrcode::QrCode;
use serde_json::Value;

use crate::{
    did::{DIDDocument, DidCommInvitation},
    key::keypair_from_seed,
};

pub async fn create_qr_code(id: String, base_url: String) -> Result<String> {
    let didcomm_inv = format!("{}/inv?id={}", base_url, id);
    let code = QrCode::new(&didcomm_inv)?;
    let string = code
        .render::<char>()
        .quiet_zone(true)
        .module_dimensions(2, 1)
        .dark_color('█')
        .light_color(' ')
        .build();
    Ok(format!("{}\n{}", string, didcomm_inv))
}

pub fn create_oob_url(id: &str, base_url: String, domain: String, seed: String) -> Result<String> {
    let (_, verify_key) = keypair_from_seed(&seed);
    let oob_data =
        DidCommInvitation::new(id.to_string(), base_url.clone(), Some(domain), verify_key);
    let base64_oob_data = general_purpose::STANDARD.encode(&serde_json::to_string(&oob_data)?);
    let url = format!(
        "{}/ssi?oob={}&id={}",
        base_url,
        urlencoding::encode(&base64_oob_data),
        id
    );
    Ok(url)
}

pub fn create_did_document(id: String, domain: String, seed: String) -> Result<String> {
    let (_, verify_key) = keypair_from_seed(&seed);
    let did_doc = DIDDocument::new(id, verify_key, domain);
    Ok(serde_json::to_string_pretty(&did_doc)?)
}

pub fn handle_didcomm(value: Value, signing_key: SigningKey) -> Result<String> {
    let jwe_string: String = serde_json::to_string(&value)?;

    tracing::debug!("jwe_string: {}", jwe_string);

    let private = signing_key.to_bytes();

    match Message::receive(&jwe_string, Some(&private), None, None) {
        Ok(message) => Ok(serde_json::to_string_pretty(&message)?),
        Err(e) => {
            tracing::error!("Error: {:?}", e);
            Err(anyhow::anyhow!("Error: {:?}", e))
        }
    }
}
