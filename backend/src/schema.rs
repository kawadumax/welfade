table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        session_token -> Nullable<Varchar>,
    }
}
