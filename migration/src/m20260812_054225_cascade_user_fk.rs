use entity::{access_token, user};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::sea_query::ForeignKeyAction;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260812_054225_cascade_user_fk"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Schema-builder creates the FK with the default NO ACTION. Recreate
        // it with ON DELETE/UPDATE CASCADE so deleting a user cascades to tokens.
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-access_tokens-user_id")
                    .table(access_token::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-access_tokens-user_id")
                    .from(access_token::Entity, access_token::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-access_tokens-user_id")
                    .table(access_token::Entity)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-access_tokens-user_id")
                    .from(access_token::Entity, access_token::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .to_owned(),
            )
            .await
    }
}
