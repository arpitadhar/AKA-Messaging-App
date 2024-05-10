// @generated automatically by Diesel CLI.

diesel::table! {
    conversations (id) {
        id -> Text,
        user1_id -> Text,
        user2_id -> Text,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Text,
        conversation_id -> Text,
        message -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        //is_admin -> Nullable<Bool>,
        username -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        //reated_at -> Timestamp,
        //p_reset -> Nullable<Bool>,
        token -> Text,
    }
}

diesel::joinable!(messages -> conversations (conversation_id));

diesel::allow_tables_to_appear_in_same_query!(
    conversations,
    messages,
    users,
);
