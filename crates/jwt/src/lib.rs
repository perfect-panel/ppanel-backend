use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: i64,
    pub iat: i64,
    #[serde(rename = "UserId")]
    pub user_id: i64,
    #[serde(rename = "SessionId")]
    pub session_id: String,
    #[serde(rename = "LoginType")]
    pub login_type: String,
}

impl Claims {
    pub fn new(user_id: i64, session_id: String, login_type: String) -> (Self, i64) {
        let now = chrono_now();
        let seconds = 604800;
        (
            Self {
                iat: now,
                exp: now + seconds,
                user_id,
                session_id,
                login_type,
            },
            seconds,
        )
    }
}

fn chrono_now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

pub fn generate_token(claims: &Claims, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    encode(
        &Header::default(),
        claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn validate_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
