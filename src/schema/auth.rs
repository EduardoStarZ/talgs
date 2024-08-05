diesel::table! {
    groups (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
        group -> Integer,
        email -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    users,
);
