use entity::{access_token, user};
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::sea_query::{Alias, extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260812_054225_init"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Entity-first: build the whole schema from the entities. Table order,
        // the `userrole` PG enum, FKs and unique indexes are all derived here.
        let db = manager.get_connection();
        db.get_schema_builder()
            .register(user::Entity)
            .register(access_token::Entity)
            .apply(db)
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(access_token::Entity).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(user::Entity).to_owned())
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(Alias::new("userrole"))
                    .to_owned(),
            )
            .await
    }
}
