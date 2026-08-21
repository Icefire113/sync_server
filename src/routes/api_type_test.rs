mod tests {

    #[test]
    fn api_role_matches_entity_role_serialization() {
        use api_types::Role;
        for api in [Role::Banned, Role::User, Role::Admin] {
            let db = match api {
                Role::Banned => entity::user::Role::Banned,
                Role::User => entity::user::Role::User,
                Role::Admin => entity::user::Role::Admin,
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
        use api_types::Role;
        for api in [Role::Banned, Role::User, Role::Admin] {
            let db: entity::user::Role = api.into();
            let back: Role = db.into();
            assert_eq!(api, back);
        }
    }
}
