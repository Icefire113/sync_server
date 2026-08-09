pub mod user {
    use chrono::{DateTime, Utc};
    use sea_orm::{ActiveValue::Set, entity::prelude::*};

    #[derive(
        Debug, Clone, Copy, PartialEq, Default, DeriveActiveEnum, EnumIter, Eq, PartialOrd, Ord,
    )]
    #[sea_orm(db_type = "Enum", enum_name = "userrole")]
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
        pub access_tokens: HasMany<super::access_token::Entity>,
    }

    // https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-format/#active-model-behavior
    impl ActiveModelBehavior for ActiveModel {
        /// This is the defaults for the rows
        fn new() -> Self {
            Self {
                enabled: Set(false),
                role: Set(Role::default()),
                // TODO: double check that this should not be a before_save hook
                created_at: Set(Utc::now()),
                ..ActiveModelTrait::default()
            }
        }
    }
}

pub mod access_token {
    use chrono::{DateTime, Utc};
    use sea_orm::{ActiveValue::Set, entity::prelude::*};

    #[sea_orm::model]
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "access_tokens")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        /// sha2-256 hash of the token
        #[sea_orm(indexed)]
        pub token_hash: Vec<u8>,
        pub user_id: i64,
        #[sea_orm(belongs_to, from = "user_id", to = "id")]
        pub user: BelongsTo<super::user::Entity>,
        pub name: String,
        pub created_at: DateTime<Utc>,
        pub expires_at: DateTime<Utc>,
        pub revoked_at: Option<DateTime<Utc>>,
        pub last_used_at: Option<DateTime<Utc>>,
    }

    impl ActiveModelBehavior for ActiveModel {
        fn new() -> Self {
            Self {
                name: Set("Default Access Token".to_owned()),
                revoked_at: Set(None),
                last_used_at: Set(None),
                created_at: Set(Utc::now()),
                ..ActiveModelTrait::default()
            }
        }
    }
}
