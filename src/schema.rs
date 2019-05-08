table! {
    masses (id) {
        id -> Nullable<Integer>,
        user_id -> Nullable<Integer>,
        name -> Varchar,
        mass -> Varchar,
        last_modified -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Varchar,
        hash -> Varchar,
        salt -> Varchar,
        email -> Varchar,
        created -> Timestamp,
    }
}
