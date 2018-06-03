table! {
    invites (id) {
        id -> Int4,
        workshop_id -> Int4,
        email -> Varchar,
        attending -> Bool,
        pending -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    salts (id) {
        id -> Int4,
        salt -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    workshops (id) {
        id -> Int4,
        name -> Varchar,
        organizer -> Int4,
        description -> Varchar,
        location -> Varchar,
        event_date -> Timestamp,
        start_time -> Timestamp,
        end_time -> Timestamp,
        private -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(invites -> workshops (workshop_id));
joinable!(workshops -> users (organizer));

allow_tables_to_appear_in_same_query!(invites, salts, users, workshops,);
