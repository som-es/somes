diesel::table! {
    user (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password_hash -> Text,
    }
}
