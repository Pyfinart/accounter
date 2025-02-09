use sqlx::types::time::OffsetDateTime;
use crate::domain::user::User;
use crate::repositories::user_repository::UserRepositoryImpl;
use crate::repositories::user_repository::UserRepository;

pub struct UserService {
    user_repository: UserRepositoryImpl,
}

impl UserService {
    pub fn new(user_repository: UserRepositoryImpl) -> Self {
        UserService { user_repository }
    }

    /// Create a new user
    pub async fn create_user(&self, username: String, password: String) -> Result<u64, String> {
        // Validate input parameters
        if username.is_empty() || password.is_empty() {
            return Err("Username and password cannot be empty".to_string());
        }

        if let Ok(had) = self.user_repository.check_username_exists(&username).await {
            if had {
                return Err("Username already exists".to_string());
            }
        }

        // todo Hash the password

        // Create a new user record in the database
        let user = User {
            user_id: 0,
            username,
            password_hash: password,
            create_time: OffsetDateTime::now_utc(),
            update_time: OffsetDateTime::now_utc(),
        };
        //Save the user record to the database
        self.user_repository.create_user(&user).await
    }

    /// User login
    pub async fn user_login(&self, username: String, password: String) -> Result<i8, String>{
        // sql注入校验
        if username.contains("'") || password.contains("'") {
            return Err("Invalid input".to_string());
        }
        Ok(0)
    }
}