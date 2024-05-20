// @generated automatically by Diesel CLI.

diesel::table! {
    conversations (id) {
        id -> Text,
        user1_id -> Text,
        user2_id -> Text,
    }
}

diesel::table! {
    flagged (id) {
        id -> Int4,
        email -> Text,
        reason -> Text,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Text,
        message -> Text,
        conversation_id -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        img_url -> Text,
        is_admin -> Nullable<Bool>,
    }
}

diesel::joinable!(messages -> conversations (conversation_id));

diesel::allow_tables_to_appear_in_same_query!(
    conversations,
    flagged,
    messages,
    users,
);
