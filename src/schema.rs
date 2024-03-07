// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Text,
        message -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        is_admin -> Nullable<Bool>,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
