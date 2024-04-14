use ed25519_dalek::SECRET_KEY_LENGTH;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use did_key::{generate, DIDCore, Ed25519KeyPair, KeyMaterial};

mod claims;
mod cred_offer;
mod oauth_grants;
mod pre_authorized_code;

pub use claims::Claims;
pub use cred_offer::CredOffer;
pub use oauth_grants::OAuthGrants;
pub use pre_authorized_code::PreAuthorizedCode;

pub fn create_jwt(claims: Claims, seed: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let keypair = generate::<Ed25519KeyPair>(Some(seed.as_bytes()));

    let did = keypair.get_did_document(Default::default()).id;

    let doc =
        ring::signature::Ed25519KeyPair::generate_pkcs8(&ring::rand::SystemRandom::new()).unwrap();
    let encoding_key = EncodingKey::from_ed_der(doc.as_ref());

    let header = Header {
        alg: Algorithm::EdDSA,
        kid: Some(format!("{}#{}", did, did.replace("did:key:", ""))),
        ..Default::default()
    };

    encode(&header, &claims, &encoding_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use ed25519_dalek::{Signer, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH};
    use jsonwebtoken::DecodingKey;

    #[test]
    fn test_create_jwt() {
        let claims = Claims {
            iss: "https://example.com".to_string(),
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::days(1)).timestamp(),
            id: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
        };

        let secret = [0u8; SECRET_KEY_LENGTH];
        let jwt = create_jwt(claims.clone(), &secret).unwrap();

        let claims2 = jsonwebtoken::decode::<Claims>(
            &jwt,
            &DecodingKey::from_secret(&secret),
            &jsonwebtoken::Validation::new(Algorithm::HS256),
        )
        .unwrap()
        .claims;

        assert_eq!(claims, claims2);
    }
}
