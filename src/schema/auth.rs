// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
        group -> SmallInt,
        email -> Text,
    }
}
