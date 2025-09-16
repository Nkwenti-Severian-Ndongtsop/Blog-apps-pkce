use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestClaims {
    pub sub: String,
    pub roles: Vec<String>,
    pub exp: i64,
    pub iat: i64,
}

/// Generate a test JWT token for local development
pub fn generate_test_token() -> String {
    let now = Utc::now();
    let exp = now + Duration::hours(1);

    let claims = TestClaims {
        sub: "admin".to_string(),
        roles: vec!["author".to_string()],
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    // Use the same secret as the rest of the application
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-256-bit-secret".to_string());
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());

    encode(&Header::default(), &claims, &encoding_key).expect("Failed to generate test token")
}

/// Validate test token (for local development only)
pub fn validate_test_token(token: &str) -> Result<TestClaims, jsonwebtoken::errors::Error> {
    use jsonwebtoken::{decode, DecodingKey, Validation};

    // Use the same secret as the rest of the application
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-256-bit-secret".to_string());
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    let mut validation = Validation::default();
    validation.validate_exp = false; // Don't validate expiration for test tokens

    let token_data = decode::<TestClaims>(token, &decoding_key, &validation)?;

    Ok(token_data.claims)
}
