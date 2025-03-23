use crate::repository::user::user_repository::UserRepositoryImpl;
use sea_orm::DatabaseConnection;
use stariver_domain::user::aggregate::User;
use stariver_domain::user::repository::UserRepository;
use stariver_infrastructure::model::err::CodedErr;

pub struct UserApplication {
    repo: UserRepositoryImpl,
}

impl UserApplication {
    /// 新建
    pub fn new(conn: &'static DatabaseConnection) -> UserApplication {
        UserApplication {
            repo: UserRepositoryImpl::new(conn),
        }
    }

    pub async fn register_user(&self, username: &str, password: &str) -> Result<User, CodedErr> {
        // todo add publish register event
        let user = User::new_with_username_and_password(username, password, "stariver");
        if user.is_err() {
            return Err(CodedErr::new_with_system_self_reason(
                user.unwrap_err().to_string(),
            ));
        }
        self.repo
            .insert(user.unwrap())
            .await
            .map_err(|e| CodedErr::new("B0000".to_string(), e.to_string()))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, CodedErr> {
        self.repo
            .find_by_username(username)
            .await
            .map_err(|err| CodedErr::new("B0000".to_string(), err.to_string()))
    }
}
