// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "level"))]
    pub struct Level;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;

    users (id) {
        id -> Int4,
        firstname -> Text,
        lastname -> Text,
        bio -> Text,
        email -> Text,
        password -> Text,
        deleted_at -> Nullable<Timestamp>,
        role -> Nullable<Role>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Level;

    users_has_skills (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Text,
        level -> Level,
    }
}

diesel::joinable!(users_has_skills -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    users,
    users_has_skills,
);
