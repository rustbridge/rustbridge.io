table! {
    sessions (id) {
        id -> Int4,
        session_key -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    sessions,
    users,
);
