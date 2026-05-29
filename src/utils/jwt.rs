use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};

use crate::schemas::auth::Claims;
use crate::config::config::Config;


pub fn generate_jwt(user_id: u64, config: &Config) -> String {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;


    let claims = Claims{
        exp: exp,
        user_id: user_id,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret_key_bytes()),
    )
    .expect("JWT encode failed")
}
