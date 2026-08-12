use entity::access_token;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260812_055853_unique_token_hash"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // A token hash must never repeat, so back it with a unique index. Adding
        // a unique-only index (instead of a constraint) lets `down` drop it cleanly.
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-access_tokens-token_hash_unique")
                    .table(access_token::Entity)
                    .col(access_token::Column::TokenHash)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-access_tokens-token_hash_unique")
                    .table(access_token::Entity)
                    .to_owned(),
            )
            .await
    }
}
