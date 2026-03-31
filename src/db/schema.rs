pub mod user {
    use sea_orm::{ActiveValue::Set, entity::prelude::*};

    #[sea_orm::model]
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        // i64 cause postgres doenst support unsigned ints? what the helly
        #[sea_orm(primary_key)]
        pub id: i64,
        #[sea_orm(unique)]
        pub username: String,
        pub enabled: bool,
        #[sea_orm(has_many)]
        pub tracked_files: HasMany<super::tracked_file::Entity>,
    }

    // https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-format/#active-model-behavior
    impl ActiveModelBehavior for ActiveModel {
        /// This is the defaults for the rows
        fn new() -> Self {
            Self {
                enabled: Set(false),
                ..ActiveModelTrait::default()
            }
        }
    }
}

pub mod tracked_file {
    use sea_orm::entity::prelude::*;

    #[sea_orm::model]
    #[derive(DeriveEntityModel, Clone, Debug, PartialEq)]
    #[sea_orm(table_name = "tracked_files")]
    pub struct Model {
        // i64 cause postgres doenst support unsigned ints? what the helly
        #[sea_orm(primary_key)]
        pub id: i64,
        pub user_id: i64,
        #[sea_orm(belongs_to, from = "user_id", to = "id")]
        pub user: HasOne<super::user::Entity>,
        pub discriminator: String,
    }

    // https://www.sea-ql.org/SeaORM/docs/generate-entity/entity-format/#active-model-behavior
    impl ActiveModelBehavior for ActiveModel {
        /// This is the defaults for the rows
        fn new() -> Self {
            Self {
                ..ActiveModelTrait::default()
            }
        }
    }
}
