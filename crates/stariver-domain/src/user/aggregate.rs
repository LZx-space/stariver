use anyhow::Error;
use argon2::password_hash::PasswordHashString;
use serde::{Deserialize, Serialize};
use stariver_infrastructure::security::authentication::util::{
    from_hashed_password, hash_password, verify_password,
};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: Username,
    pub password: Password,
    pub state: State,
    pub created_at: OffsetDateTime,
    pub login_events: Vec<LoginEvent>,
}

impl User {
    pub fn new_with_username_and_password(
        username: &str,
        password: &str,
        pwd_salt: &str,
    ) -> Result<Self, Error> {
        let username = Username::new(username)?;
        let password = Password::new_by_raw_password(password, pwd_salt)?;
        Ok(User {
            id: Uuid::now_v7(),
            username,
            password,
            state: Default::default(),
            created_at: OffsetDateTime::now_utc(),
            login_events: vec![],
        })
    }
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Password(String);

impl Password {
    pub fn new_by_raw_password(password: &str, salt: &str) -> Result<Self, Error> {
        hash_password(password, salt)
            .map_err(|e| Error::msg(e.to_string()))
            .map(|e| Self(e.to_string()))
    }

    pub fn new_by_hashed_password_string(hashed_password: &str) -> Result<Self, Error> {
        from_hashed_password(hashed_password)
            .map_err(|e| Error::msg(e.to_string()))
            .map(|e| Self(e.to_string()))
    }

    pub fn verify_password(&self, input: &str) -> Result<(), Error> {
        let password_hash_string =
            PasswordHashString::new(&self.0).map_err(|e| Error::msg(e.to_string()))?;
        verify_password(input, &password_hash_string)
            .map(|_| ())
            .map_err(|e| Error::msg("cannot verify password"))
    }

    pub fn hashed_password_string(&self) -> &str {
        &self.0
    }
}

// -----entity LoginEvent---------------------------------------------------

#[derive(Debug, Serialize)]
pub struct LoginEvent {
    pub login_at: OffsetDateTime,
    pub ip: String,
    pub result: LoginResult,
}

#[derive(Debug, Serialize)]
pub enum LoginResult {
    Success,
    Failure(String),
}
