use super::schema::*;
use crate::database::Database;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Debug, Identifiable)]
pub struct ActiveStorageAttachment {
    pub id: i64,
    pub name: String,
    pub record_type: String,
    pub record_id: i64,
    pub blob_id: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Debug, Identifiable)]
pub struct ActiveStorageBlob {
    pub id: i64,
    pub key: String,
    pub filename: String,
    pub content_type: Option<String>,
    pub metadata: Option<String>,
    pub service_name: String,
    pub byte_size: i64,
    pub checksum: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Segment {
    pub id: i64,
    pub variant_id: i64,
    pub position: i32,
    pub duration: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Variant {
    pub id: i64,
    pub video_id: i64,
    pub public_id: String,
    pub height: i32,
    pub width: i32,
    pub bitrate: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Video {
    pub id: i64,
    pub title: Option<String>,
    pub published: Option<bool>,
    pub status: Option<i32>,
    pub source: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub public_id: String,
    pub user_id: i64,
}

#[derive(Queryable, Debug)]
pub struct VideoStreamEvent {
    pub id: i64,
    pub video_id: i64,
    pub user_id: i64,
    pub duration: f64,
    pub session_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Variant {
    pub fn find_by_public_id(pid: &str) -> QueryResult<Variant> {
        use crate::schema::variants::dsl::*;

        variants
            .filter(public_id.eq(pid))
            .first(&Database::connection())
    }

    pub fn by_video(video: &Video) -> Vec<Variant> {
        use crate::schema::variants::dsl::*;

        let connection = Database::connection();

        variants
            .filter(video_id.eq(video.id))
            .load::<Variant>(&connection)
            .expect("Error loading Video")
    }
}

impl Segment {
    pub fn find_by_position(variant: i64, pos: i32) -> QueryResult<Segment> {
        use crate::schema::segments::dsl::*;

        let connection = Database::connection();

        segments
            .filter(variant_id.eq(variant))
            .filter(position.eq(pos))
            .first(&connection)
    }

    pub fn by_variant(variant: &Variant) -> Vec<Segment> {
        use crate::schema::segments::dsl::*;

        let connection = Database::connection();

        segments
            .filter(variant_id.eq(variant.id))
            .load::<Segment>(&connection)
            .expect("Error loading Segment")
    }
}

impl ActiveStorageAttachment {
    pub fn find_by_segment(segment_id: i64) -> QueryResult<ActiveStorageAttachment> {
        use crate::schema::active_storage_attachments::dsl::*;

        let connection = Database::connection();

        active_storage_attachments
            .filter(record_type.eq("Segment"))
            .filter(record_id.eq(segment_id))
            .first(&connection)
    }
}

impl ActiveStorageBlob {
    pub fn find_by_id(record_id: i64) -> ActiveStorageBlob {
        use crate::schema::active_storage_blobs::dsl::*;

        let connection = Database::connection();

        match active_storage_blobs.find(record_id).first(&connection) {
            Ok(blob) => blob,
            Err(_) => panic!("Error querying Blob."),
        }
    }
}

impl Video {
    pub fn find_by_public_id(pid: &str) -> QueryResult<Video> {
        use crate::schema::videos::dsl::*;

        let connection = Database::connection();

        videos.filter(public_id.eq(pid)).first(&connection)
    }
}

impl VideoStreamEvent {
    pub fn log_event(variant_object: &Variant, segment_duration: f64, session: &str) -> usize {
        use crate::schema::video_stream_events::dsl::*;

        let connection = Database::connection();

        let now = std::time::SystemTime::now();

        diesel::insert_into(video_stream_events)
            .values((
                video_id.eq(variant_object.video_id),
                duration.eq(segment_duration),
                session_id.eq(session),
                user_id.eq(1),
                created_at.eq(now),
                updated_at.eq(now),
            ))
            .execute(&connection)
            .expect("Failed to insert VideoWatchEvent.")
    }
}
