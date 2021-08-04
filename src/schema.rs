table! {
    access_tokens (id) {
        id -> Int8,
        name -> Varchar,
        access_token -> Varchar,
        public_id -> Varchar,
        user_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    active_storage_attachments (id) {
        id -> Int8,
        name -> Varchar,
        record_type -> Varchar,
        record_id -> Int8,
        blob_id -> Int8,
        created_at -> Timestamp,
    }
}

table! {
    active_storage_blobs (id) {
        id -> Int8,
        key -> Varchar,
        filename -> Varchar,
        content_type -> Nullable<Varchar>,
        metadata -> Nullable<Text>,
        service_name -> Varchar,
        byte_size -> Int8,
        checksum -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    active_storage_variant_records (id) {
        id -> Int8,
        blob_id -> Int8,
        variation_digest -> Varchar,
    }
}

table! {
    ar_internal_metadata (key) {
        key -> Varchar,
        value -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    schema_migrations (version) {
        version -> Varchar,
    }
}

table! {
    segments (id) {
        id -> Int8,
        variant_id -> Int8,
        position -> Int4,
        duration -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    signature_keys (id) {
        id -> Int8,
        name -> Varchar,
        signature_key -> Varchar,
        public_id -> Varchar,
        user_id -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int8,
        email -> Varchar,
        encrypted_password -> Varchar,
        remember_created_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    variants (id) {
        id -> Int8,
        video_id -> Int8,
        public_id -> Varchar,
        height -> Int4,
        width -> Int4,
        bitrate -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    video_stream_events (id) {
        id -> Int8,
        video_id -> Int8,
        user_id -> Int8,
        duration -> Float8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        session_id -> Nullable<Varchar>,
    }
}

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

table! {
    webhook_subscriptions (id) {
        id -> Int8,
        topic -> Varchar,
        url -> Varchar,
        user_id -> Int8,
        public_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(access_tokens -> users (user_id));
joinable!(active_storage_attachments -> active_storage_blobs (blob_id));
joinable!(active_storage_variant_records -> active_storage_blobs (blob_id));
joinable!(segments -> variants (variant_id));
joinable!(signature_keys -> users (user_id));
joinable!(variants -> videos (video_id));
joinable!(video_stream_events -> users (user_id));
joinable!(video_stream_events -> videos (video_id));
joinable!(videos -> users (user_id));
joinable!(webhook_subscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    active_storage_attachments,
    active_storage_blobs,
    active_storage_variant_records,
    ar_internal_metadata,
    schema_migrations,
    segments,
    signature_keys,
    users,
    variants,
    video_stream_events,
    videos,
    webhook_subscriptions,
);
