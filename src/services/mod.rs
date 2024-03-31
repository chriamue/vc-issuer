use anyhow::Result;
use base64::engine::general_purpose;
use base64::Engine as _;
use qrcode::QrCode;

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

pub fn create_oob_url(id: &str, base_url: String) -> Result<String> {
    let oob_data = DidCommInvitation::new(id.to_string(), base_url.clone());
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
