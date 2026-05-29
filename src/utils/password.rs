use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString,
    },
    Argon2
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(hash)
}


pub fn check_password(password: &str, hash_password: &str) -> Result<bool, argon2::password_hash::Error> {
    let parser_hash = PasswordHash::new(hash_password)?;

    Ok(
        Argon2::default()
            .verify_password(password.as_bytes(), &parser_hash)
            .is_ok()
    )
}
