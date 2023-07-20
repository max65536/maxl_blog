// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        content -> Text,
        published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
