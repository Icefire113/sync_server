pub use sea_orm_migration::prelude::*;

mod m20260812_054225_cascade_user_fk;
mod m20260812_054225_init;
mod m20260812_055853_unique_token_hash;
mod m20260818_025319_add_tracked_files;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260812_054225_cascade_user_fk::Migration),
            Box::new(m20260812_054225_init::Migration),
            Box::new(m20260812_055853_unique_token_hash::Migration),
            Box::new(m20260818_025319_add_tracked_files::Migration),
        ]
    }
}
