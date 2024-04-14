use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Claims {
    pub iss: String,
    pub iat: i64,
    pub exp: i64,
    pub id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_claims() {
        let claims = Claims {
            iss: "https://example.com".to_string(),
            iat: 1234567890,
            exp: 1234567890,
            id: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
        };

        let json = serde_json::to_string(&claims).unwrap();
        let claims2: Claims = serde_json::from_str(&json).unwrap();

        assert_eq!(claims, claims2);
    }
}
