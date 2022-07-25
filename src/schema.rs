table! {
    message (id) {
        id -> Int4,
        name -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    wiggles_user (email) {
        id -> Int4,
        name -> Varchar,
        password -> Varchar,
        email -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    message,
    wiggles_user,
);
