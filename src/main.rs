mod database;
mod models;
mod schema;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate rocket;

use models::*;
use regex::Regex;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::response::content::Custom;
use rocket::response::status::NotFound;
use std::path::Path;

#[get("/segments/<file_name>")]
async fn segment(file_name: &str) -> Result<Custom<NamedFile>, NotFound<String>> {
    let regex = Regex::new(r"([a-zA-Z0-9]{10})_(\d+)\.ts").unwrap();
    let capture_groups = regex.captures(file_name).unwrap();

    let variant_pid = capture_groups.get(1).map_or("", |m| m.as_str());
    let segment_position = capture_groups.get(2).map_or("", |m| m.as_str());

    let variants = Variant::by_public_id(variant_pid);

    let first_variant = variants.first();

    let variant = match first_variant {
        Some(variant) => variant,
        None => return Result::Err(NotFound("Variant not found.".to_string())),
    };

    let segments = Segment::by_position(variant.id, segment_position.parse::<i32>().unwrap());

    let first_segment = segments.first();

    let segment = match first_segment {
        Some(segment) => segment,
        None => return Result::Err(NotFound("Segment not found.".to_string())),
    };

    let attachment = ActiveStorageAttachment::by_segment(segment.id);

    let first_attachment = attachment.first();

    let attachment = match first_attachment {
        Some(attachment) => attachment,
        None => return Result::Err(NotFound("Attachment not found.".to_string())),
    };

    let blob = ActiveStorageBlob::find_by_id(attachment.blob_id);

    let key = blob.key;

    let path = format!(
        "/Users/gui/dev/albuvideo-api/storage/{}/{}/{}",
        &key[..=1],
        &key[2..=3],
        key
    );

    let named_file = NamedFile::open(Path::new(&path)).await;

    let content_type = ContentType::new("video", "x-MP2T");

    match named_file {
        Ok(named_file) => Result::Ok(Custom(content_type, named_file)),
        Err(e) => Result::Err(NotFound(e.to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![segment])
}
