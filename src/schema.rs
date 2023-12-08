// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Int4,
        created_at -> Date,
        verified -> Bool,
    }
}

diesel::table! {
    api_key (id) {
        id -> Int4,
        #[max_length = 255]
        key -> Varchar,
        expires -> Date,
        created_at -> Date,
        user_id -> Int4,
        revoked -> Bool,
    }
}

diesel::joinable!(api_key -> account (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    api_key,
);
