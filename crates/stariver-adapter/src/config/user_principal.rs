use sea_orm::DatabaseConnection;
use sea_orm::prelude::async_trait::async_trait;
use serde::{Deserialize, Serialize};
use stariver_application::repository::user::user_repository::UserRepositoryImpl as DomainUserRepoImpl;
use stariver_domain::user::aggregate::{Password, Username};
use stariver_domain::user::repository::UserRepository as DomainUserRepo;
use stariver_infrastructure::security::authentication::core::authenticator::{
    AuthenticationError, Authenticator,
};
use stariver_infrastructure::security::authentication::core::credential::{Credential, Ctx};
use stariver_infrastructure::security::authentication::core::principal::{
    Principal, SimpleAuthority,
};
use stariver_infrastructure::security::authentication::password_hasher::{
    from_hashed_password, verify_password,
};
use std::fmt::Debug;
use tracing::{error, info};

pub struct UsernamePasswordCredential {
    username: String,
    password: String,
}

impl Credential for UsernamePasswordCredential {
    fn request_details(&self) -> Ctx {
        Ctx {}
    }
}

impl UsernamePasswordCredential {
    pub fn new(username: String, password: String) -> Result<Self, AuthenticationError> {
        if username.is_empty() {
            return Err(AuthenticationError::UsernameEmpty);
        }
        if password.is_empty() {
            return Err(AuthenticationError::PasswordEmpty);
        }
        Ok(UsernamePasswordCredential { username, password })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    username: Username,
    #[serde(skip_serializing)]
    password: Password,
    #[serde(default)]
    authorities: Vec<SimpleAuthority>,
}

impl Principal for User {
    type Id = Username;
    type Authority = SimpleAuthority;

    fn id(&self) -> &Self::Id {
        &self.username
    }

    fn authorities(&self) -> Vec<&Self::Authority> {
        vec![]
    }
}

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, user_id: &String) -> Result<User, AuthenticationError>;
}

pub struct UserRepositoryImpl {
    delegate: DomainUserRepoImpl,
}

impl UserRepositoryImpl {
    pub fn new(conn: &'static DatabaseConnection) -> Self {
        UserRepositoryImpl {
            delegate: DomainUserRepoImpl::new(conn),
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, user_id: &String) -> Result<User, AuthenticationError> {
        let user = self.delegate.find_by_username(user_id).await.map_err(|e| {
            error!("Failed to find user by username: {}", e);
            AuthenticationError::Unknown
        })?;
        match user {
            Some(u) => Ok(User {
                username: u.username,
                password: u.password,
                authorities: vec![],
            }),
            None => {
                info!("User not found with username: {}", user_id);
                Err(AuthenticationError::UsernameNotFound)
            }
        }
    }
}

pub struct UserAuthenticator {
    user_repository: UserRepositoryImpl,
}

impl UserAuthenticator {
    pub fn new(repo: UserRepositoryImpl) -> UserAuthenticator {
        UserAuthenticator {
            user_repository: repo,
        }
    }
}

#[async_trait]
impl Authenticator for UserAuthenticator {
    type Credential = UsernamePasswordCredential;
    type Principal = User;

    async fn authenticate(
        &self,
        credential: &Self::Credential,
    ) -> Result<Self::Principal, AuthenticationError> {
        let username = &credential.username;
        let password = &credential.password;
        // 查找用户
        let user = self.user_repository.find_by_id(username).await?;
        // 验证密码
        let password_hash_string = from_hashed_password(user.password.hashed_password_string())
            .map_err(|e| AuthenticationError::BadPassword)?;
        verify_password(password.as_str(), &password_hash_string)
            .map(|_| user)
            .map_err(|_| AuthenticationError::BadPassword)
    }
}
