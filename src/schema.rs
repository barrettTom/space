table! {
    requests (id) {
        id -> Text,
        data -> Text,
        time -> Timestamp,
        received -> Bool,
    }
}

table! {
    responses (id) {
        id -> Text,
        data -> Text,
        time -> Timestamp,
        request_id -> Text,
    }
}
