table! {
    videos (id) {
        id -> Int8,
        title -> Nullable<Varchar>,
        published -> Nullable<Bool>,
        status -> Nullable<Int4>,
        source -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        public_id -> Varchar,
        user_id -> Int8,
    }
}
