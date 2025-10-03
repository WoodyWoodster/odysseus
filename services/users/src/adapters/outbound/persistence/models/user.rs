use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::User;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// Implement From<Model> for User (model -> domain entity mapping)
impl From<Model> for User {
    fn from(model: Model) -> Self {
        User {
            id: model.id,
            email: model.email,
            name: model.name,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_model_to_user_conversion() {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let model = Model {
            id,
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            created_at: now,
            updated_at: now,
        };

        let user: User = model.clone().into();

        assert_eq!(user.id, id);
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
        assert_eq!(user.created_at, now);
        assert_eq!(user.updated_at, now);
    }

    #[test]
    fn test_model_to_user_preserves_all_fields() {
        let id = Uuid::new_v4();
        let created = Utc::now();
        let updated = Utc::now();

        let model = Model {
            id,
            email: "user@domain.com".to_string(),
            name: "John Doe".to_string(),
            created_at: created,
            updated_at: updated,
        };

        let user: User = model.into();

        assert_eq!(user.id, id, "ID should be preserved");
        assert_eq!(user.email, "user@domain.com", "Email should be preserved");
        assert_eq!(user.name, "John Doe", "Name should be preserved");
        assert_eq!(
            user.created_at, created,
            "Created timestamp should be preserved"
        );
        assert_eq!(
            user.updated_at, updated,
            "Updated timestamp should be preserved"
        );
    }

    #[test]
    fn test_model_to_user_with_special_characters() {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let model = Model {
            id,
            email: "test+filter@example.co.uk".to_string(),
            name: "François O'Brien-Smith".to_string(),
            created_at: now,
            updated_at: now,
        };

        let user: User = model.into();

        assert_eq!(user.email, "test+filter@example.co.uk");
        assert_eq!(user.name, "François O'Brien-Smith");
    }
}
