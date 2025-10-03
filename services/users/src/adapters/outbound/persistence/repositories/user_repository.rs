use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::domain::{CreateUserData, DomainError, DomainResult, User, UserRepository};

use super::super::models::user::{ActiveModel, Entity as UserEntity};

pub struct UserRepositoryImpl {
    db: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<User> {
        let user = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(format!("User with id {} not found", id)))?;

        Ok(user.into())
    }

    async fn create(&self, data: CreateUserData) -> DomainResult<User> {
        let now = Utc::now();
        let user_id = Uuid::new_v4();

        let user = ActiveModel {
            id: Set(user_id),
            email: Set(data.email),
            name: Set(data.name),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let inserted = user
            .insert(&self.db)
            .await
            .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(inserted.into())
    }
}
