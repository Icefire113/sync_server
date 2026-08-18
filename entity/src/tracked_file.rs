use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,

    /// xxhash3 hash of the file contents. Stored as i64 because Postgres has no
    /// u64 type; this is a bit-cast of the u64 hash (see `hash` in api-types)
    pub hash: i64,

    #[sea_orm(indexed)]
    pub name: String,

    /// The machine id that last wrote to this file
    pub last_updated_from: String,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub user_id: i64,
    #[sea_orm(belongs_to, from = "user_id", to = "id")]
    pub user: BelongsTo<crate::user::Entity>,
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            ..ActiveModelTrait::default()
        }
    }
}
