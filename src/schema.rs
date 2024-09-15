// @generated automatically by Diesel CLI.

diesel::table! {
    pages (title) {
        title -> Text,
        url -> Text,
        language -> Text,
        last_updated -> Nullable<Timestamp>,
        content -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    pages,
    users,
);
