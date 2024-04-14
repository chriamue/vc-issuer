use ed25519_dalek::SECRET_KEY_LENGTH;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

mod claims;
mod cred_offer;
mod oauth_grants;
mod pre_authorized_code;

pub use claims::Claims;
pub use cred_offer::CredOffer;
pub use oauth_grants::OAuthGrants;
pub use pre_authorized_code::PreAuthorizedCode;

pub fn create_jwt(
    claims: Claims,
    secret: &[u8; SECRET_KEY_LENGTH],
) -> Result<String, jsonwebtoken::errors::Error> {
    let encoding_key = EncodingKey::from_secret(secret);

    let header = Header {
        alg: Algorithm::HS256,
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
