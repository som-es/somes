use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, Result, SaltString,
    },
    Argon2,
};
use once_cell::sync::Lazy;

static PEPPER: Lazy<String> = Lazy::new(|| {
    let pepper = std::fs::read_to_string("./src/hash/pepper_secret").unwrap_or_default();
    if pepper.is_empty() {
        log::info!("Pepper value is empty");
    }
    pepper
});

pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(format!("{password}{}", PEPPER.as_str()).as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Ok(Argon2::default()
        .verify_password(
            format!("{password}{}", PEPPER.as_str()).as_bytes(),
            &parsed_hash,
        )
        .is_ok())
}

#[cfg(test)]
mod tests {
    use argon2::password_hash::SaltString;
    use rand_core::OsRng;

    use super::{hash_password, verify_password};

    #[test]
    fn test_hash_password() -> argon2::password_hash::Result<()> {
        let password = "hunter42";
        let password_hash = hash_password(password)?;

        assert!(verify_password(password, &password_hash)?);

        Ok(())
    }

    #[test]
    fn test_salt_string() {
        let salt = SaltString::generate(&mut OsRng);
        println!("salt: {salt}")
    }
}
