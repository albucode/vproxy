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

impl Variant {
    pub fn by_public_id(pid: &str) -> Vec<Variant> {
        use crate::schema::variants::dsl::*;

        let connection = Database::connection();

        variants
            .filter(public_id.eq(pid))
            .limit(1)
            .load::<Variant>(&connection)
            .expect("Error loading Variant")
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
    pub fn by_position(variant: i64, pos: i32) -> Vec<Segment> {
        use crate::schema::segments::dsl::*;

        let connection = Database::connection();

        segments
            .filter(variant_id.eq(variant))
            .filter(position.eq(pos))
            .limit(1)
            .load::<Segment>(&connection)
            .expect("Error loading Segment")
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
    pub fn by_segment(segment_id: i64) -> Vec<ActiveStorageAttachment> {
        use crate::schema::active_storage_attachments::dsl::*;

        let connection = Database::connection();

        active_storage_attachments
            .filter(record_type.eq("Segment"))
            .filter(record_id.eq(segment_id))
            .limit(1)
            .load::<ActiveStorageAttachment>(&connection)
            .expect("Error loading ActiveStorageAttachment")
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
    pub fn by_public_id(pid: &str) -> Vec<Video> {
        use crate::schema::videos::dsl::*;

        let connection = Database::connection();

        videos
            .filter(public_id.eq(pid))
            .limit(1)
            .load::<Video>(&connection)
            .expect("Error loading Video")
    }
}
