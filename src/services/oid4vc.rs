use crate::oid4vc::{create_jwt, Claims, CredOffer, OAuthGrants, PreAuthorizedCode};
use anyhow::Result;
use chrono::{Duration, Utc};
use ed25519_dalek::SECRET_KEY_LENGTH;
use qrcode::QrCode;

pub async fn create_qr_code(id: String, base_url: String) -> Result<String> {
    let credential_offer_uri = &format!("{}/creds/{}", base_url, id);
    let inv = format!(
        "openid-credential-offer://?credential_offer_uri={}",
        urlencoding::encode(credential_offer_uri)
    );
    let code = QrCode::new(&inv)?;
    let string = code
        .render::<char>()
        .quiet_zone(true)
        .module_dimensions(2, 1)
        .dark_color('█')
        .light_color(' ')
        .build();
    Ok(format!("{}\n{}", string, inv))
}

pub fn create_cred_offer(id: String, domain: String, seed: String) -> Result<CredOffer> {
    let claims = Claims {
        iss: "https://example.com".to_string(),
        iat: Utc::now().timestamp(),
        exp: (Utc::now() + Duration::days(1)).timestamp(),
        id: "1234567890".to_string(),
        session_id: "1234567890".to_string(),
    };

    let secret = [0u8; SECRET_KEY_LENGTH];
    let jwt = create_jwt(claims.clone(), &secret).unwrap();

    let cred_offer = CredOffer {
        grants: OAuthGrants {
            pre_authorized_code: PreAuthorizedCode {
                user_pin_required: false,
                pre_authorized_code: jwt,
            },
        },
        credentials: vec!["TestCertificate".to_string()],
        credential_issuer: format!("did:{}", domain),
    };
    Ok(cred_offer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_qr_code() {
        let id = "1234567890".to_string();
        let base_url = "https://example.com".to_string();
        let qr_code = create_qr_code(id.clone(), base_url.clone()).await.unwrap();
        let lines: Vec<&str> = qr_code.lines().collect();
        let encoded_url = format!(
            "openid-credential-offer://?credential_offer_uri={}",
            urlencoding::encode(&format!("{}/creds/{}", base_url, id))
        );
        assert!(lines[lines.len() - 1].contains(&encoded_url));
    }
}
