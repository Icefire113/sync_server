pub use api_types::*;

#[cfg(test)]
mod tests {
    use crate::routes::types::Role as ApiRole;

    #[test]
    fn api_role_matches_entity_role_serialization() {
        for api in [ApiRole::Banned, ApiRole::User, ApiRole::Admin] {
            let db = match api {
                ApiRole::Banned => entity::user::Role::Banned,
                ApiRole::User => entity::user::Role::User,
                ApiRole::Admin => entity::user::Role::Admin,
            };
            assert_eq!(
                serde_json::to_string(&api).unwrap(),
                serde_json::to_string(&db).unwrap(),
                "api_types::Role serialization diverged from entity::user::Role"
            );
        }
    }

    #[test]
    fn api_role_round_trips_to_entity_role() {
        for api in [ApiRole::Banned, ApiRole::User, ApiRole::Admin] {
            let db: entity::user::Role = api.into();
            let back: ApiRole = db.into();
            assert_eq!(api, back);
        }
    }
}
