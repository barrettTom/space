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
        name -> Varchar,
        password -> Varchar,
        last_modified -> Timestamp,
    }
}
