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
        pub access_key: String,
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
    use chrono::{DateTime, Utc};
    use sea_orm::entity::prelude::*;

    #[sea_orm::model]
    #[derive(DeriveEntityModel, Clone, Debug, PartialEq)]
    #[sea_orm(table_name = "tracked_files")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uuid,
        #[sea_orm(has_many)]
        pub machine_path_mappings: HasMany<super::machine_path::Entity>,

        pub user_id: i64,
        #[sea_orm(belongs_to, from = "user_id", to = "id")]
        pub user: HasOne<super::user::Entity>,

        pub hash: u64,
        pub custom_name: Option<String>,
        pub file_last_modified: DateTime<Utc>,
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

pub mod machine_path {
    use sea_orm::entity::prelude::*;
    #[sea_orm::model]
    #[derive(DeriveEntityModel, Clone, Debug, PartialEq)]
    #[sea_orm(table_name = "machine_paths")]
    pub struct Model {
        /// The owning file id
        #[sea_orm(primary_key, auto_increment = false)]
        pub file_id: Uuid,
        #[sea_orm(belongs_to, from = "file_id", to = "id")]
        pub tracked_file: HasOne<super::tracked_file::Entity>,

        #[sea_orm(primary_key, auto_increment = false)]
        pub machine_id: String,
        pub path: String,
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
