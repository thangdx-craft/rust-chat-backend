use crate::errors::{AppError, Result};
use crate::models::user::{self, Entity as User, LoginRequest, RegisterRequest, UserResponse};
use crate::services::jwt_service::JwtService;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Clone)]
pub struct AuthService {
    db: DatabaseConnection,
    jwt_service: JwtService,
}

impl AuthService {
    pub fn new(db: DatabaseConnection, jwt_service: JwtService) -> Self {
        Self { db, jwt_service }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse> {
        // Check if user already exists
        let existing_user = User::find()
            .filter(user::Column::Email.eq(&req.email))
            .one(&self.db)
            .await?;

        if existing_user.is_some() {
            return Err(AppError::UserAlreadyExists);
        }

        // Hash password
        let password_hash = self.hash_password(&req.password)?;

        // Create user
        let new_user = user::ActiveModel {
            email: Set(req.email.clone()),
            password_hash: Set(password_hash),
            username: Set(req.username),
            created_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let user = new_user.insert(&self.db).await?;

        // Generate token
        let token = self.jwt_service.generate_token(user.id, &user.email)?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse> {
        // Find user
        let user = User::find()
            .filter(user::Column::Email.eq(&req.email))
            .one(&self.db)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        // Verify password
        self.verify_password(&req.password, &user.password_hash)?;

        // Generate token
        let token = self.jwt_service.generate_token(user.id, &user.email)?;

        Ok(AuthResponse {
            token,
            user: user.into(),
        })
    }

    fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| AppError::PasswordHashError)
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<()> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| AppError::PasswordHashError)?;

        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::InvalidCredentials)
    }
}
