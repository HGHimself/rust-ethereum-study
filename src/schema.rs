table! {
    contract (id) {
        id -> Int4,
        name -> Varchar,
        owner -> Varchar,
        address -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        active -> Bool,
    }
}
