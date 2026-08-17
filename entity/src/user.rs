use api_types::Role as ApiRole;
use chrono::{DateTime, Utc};
use sea_orm::{ActiveValue::Set, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    DeriveActiveEnum,
    EnumIter,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
)]
#[sea_orm(db_type = "Enum", enum_name = "userrole")]
#[serde(rename_all = "PascalCase")]
pub enum Role {
    #[sea_orm(string_value = "Banned")]
    Banned = 0,

    #[sea_orm(string_value = "User")]
    #[default]
    User = 1,

    #[sea_orm(string_value = "Admin")]
    Admin = 2,
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    #[sea_orm(default_value = "User")]
    pub role: Role,
    #[sea_orm(unique, indexed)]
    pub username: String,
    pub password_hash: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    #[sea_orm(has_many)]
    pub access_tokens: HasMany<crate::access_token::Entity>,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            enabled: Set(false),
            role: Set(Role::default()),
            created_at: Set(Utc::now()),
            ..ActiveModelTrait::default()
        }
    }
}

impl From<ApiRole> for Role {
    fn from(value: ApiRole) -> Self {
        match value {
            ApiRole::Banned => Self::Banned,
            ApiRole::User => Self::User,
            ApiRole::Admin => Self::Admin,
        }
    }
}

impl From<Role> for ApiRole {
    fn from(value: Role) -> Self {
        match value {
            Role::Banned => Self::Banned,
            Role::User => Self::User,
            Role::Admin => Self::Admin,
        }
    }
}
