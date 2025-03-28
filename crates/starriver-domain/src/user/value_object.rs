use anyhow::Error;
use argon2::password_hash::PasswordHashString;
use serde::{Deserialize, Serialize};
use starriver_infrastructure::security::authentication::password_hasher::{
    from_hashed_password, hash_password, verify_password,
};

#[derive(Debug, Default, Serialize)]
pub enum State {
    #[default]
    UnVerified,
    Activated,
    Disabled,
    Expired,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: &str) -> Result<Self, Error> {
        if username.len() < 3 || username.len() > 20 {
            return Err(Error::msg("must be less than 20 characters"));
        }
        Ok(Self(username.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Deserialize)]
pub struct Password {
    #[serde(skip_serializing)]
    hashed_string: String,
}

impl Password {
    pub fn new_by_raw_password(raw_password: &str) -> Result<Self, Error> {
        hash_password(raw_password)
            .map_err(|e| Error::msg(e.to_string()))
            .map(|e| Password {
                hashed_string: e.to_string(),
            })
    }

    pub fn new_by_hashed_password_string(hashed_password: &str) -> Result<Self, Error> {
        from_hashed_password(hashed_password)
            .map_err(|e| Error::msg(e.to_string()))
            .map(|e| Password {
                hashed_string: e.to_string(),
            })
    }

    pub fn verify_password(&self, input: &str) -> Result<(), Error> {
        let password_hash_string =
            PasswordHashString::new(&self.hashed_string).map_err(|e| Error::msg(e.to_string()))?;
        verify_password(input, &password_hash_string)
            .map(|_| ())
            .map_err(|e| Error::msg("cannot verify password"))
    }

    pub fn hashed_password_string(&self) -> &str {
        &self.hashed_string
    }
}

// ----- Login Event --------------------------------------
#[derive(Debug, Serialize)]
pub enum LoginResult {
    Success,
    Failure(String),
}
