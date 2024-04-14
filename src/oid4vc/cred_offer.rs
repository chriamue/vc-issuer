use super::oauth_grants::OAuthGrants;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CredOffer {
    pub grants: OAuthGrants,
    pub credentials: Vec<String>,
    pub credential_issuer: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oid4vc::PreAuthorizedCode;
    use serde_json;

    #[test]
    fn test_serialize_qr_code_data() {
        let cred_offer = CredOffer {
            grants: OAuthGrants {
                pre_authorized_code: PreAuthorizedCode {
                    user_pin_required: true,
                    pre_authorized_code: "1234567890".to_string(),
                },
            },
            credentials: vec!["TestCertificate".to_string()],
            credential_issuer: "https://example.com".to_string(),
        };

        let json = serde_json::to_string(&cred_offer).unwrap();
        let cred_offer2: CredOffer = serde_json::from_str(&json).unwrap();

        assert_eq!(cred_offer, cred_offer2);
    }
}
