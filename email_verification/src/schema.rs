table! {
    email_verification_token (id) {
        id -> Bytea,
        email -> Text,
        expires_at -> Timestamp,
        created_at -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    email_verification_token,
    user,
);
