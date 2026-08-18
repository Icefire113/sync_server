use entity::{tracked_file, user};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::sea_query::ForeignKeyAction;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260818_025319_add_tracked_files"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Entity-first: create the table from the entity (Uuid PK, unique index on
        // name, FK to users).
        let db = manager.get_connection();
        db.get_schema_builder()
            .register(tracked_file::Entity)
            .apply(db)
            .await?;

        // Schema-builder creates the FK as NO ACTION; recreate it with CASCADE so
        // deleting a user removes their tracked files, matching access_tokens.
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk-files-user_id")
                    .table(tracked_file::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-files-user_id")
                    .from(tracked_file::Entity, tracked_file::Column::UserId)
                    .to(user::Entity, user::Column::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(tracked_file::Entity).to_owned())
            .await
    }
}
