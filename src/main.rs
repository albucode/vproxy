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
    let regex = Regex::new(r"([a-zA-Z0-9]{10})_(\d+)\.ts")
        .map_err(|_| NotFound("Video not found.".to_string()))?;

    let capture_groups = regex.captures(file_name).unwrap();

    let variant_pid = capture_groups.get(1).map_or("", |m| m.as_str());
    let segment_position = capture_groups.get(2).map_or("", |m| m.as_str());

    let variant = Variant::find_by_public_id(variant_pid)
        .map_err(|_| NotFound("Variant not found.".to_string()))?;

    let segment = Segment::find_by_position(variant.id, segment_position.parse::<i32>().unwrap())
        .map_err(|_| NotFound("Segment not found.".to_string()))?;

    let attachment = ActiveStorageAttachment::find_by_segment(segment.id)
        .map_err(|_| NotFound("Attachment not found.".to_string()))?;

    let blob = ActiveStorageBlob::find_by_id(attachment.blob_id);

    let key = blob.key;

    let path = format!(
        "../albuvideo-api/storage/{}/{}/{}",
        &key[..=1],
        &key[2..=3],
        key
    );

    let named_file = NamedFile::open(Path::new(&path)).await;

    match named_file {
        Ok(named_file) => {
            VideoStreamEvent::log_event(&variant, segment.duration);

            let content_type = ContentType::new("video", "mp2t");
            Result::Ok(Custom(content_type, named_file))
        }
        Err(e) => Result::Err(NotFound(e.to_string())),
    }
}

#[get("/variants/<file_name>")]
async fn variant(file_name: &str) -> Result<Custom<String>, NotFound<String>> {
    let regex = Regex::new(r"([a-zA-Z0-9]{10}).m3u8")
        .map_err(|_| NotFound("Video not found.".to_string()))?;

    let capture_groups = match regex.captures(file_name) {
        Some(capture_groups) => capture_groups,
        None => return Result::Err(NotFound(String::from("Invalid filename."))),
    };

    let variant_pid = match capture_groups.get(1) {
        Some(variant_pid) => variant_pid.as_str(),
        None => return Result::Err(NotFound(String::from("No identifier in filename."))),
    };

    let variant = Variant::find_by_public_id(variant_pid)
        .map_err(|_| NotFound("Variant not found.".to_string()))?;

    let segments = Segment::by_variant(&variant);

    if segments.is_empty() {
        return Result::Err(NotFound("Variant has no segments.".to_string()));
    }

    let mut playlist = String::from(
        r#"#EXTM3U
#EXT-X-PLAYLIST-TYPE:VOD
#EXT-X-TARGETDURATION:6
#EXT-X-VERSION:4
#EXT-X-MEDIA-SEQUENCE:0
"#,
    );

    for segment in segments.into_iter() {
        playlist.push_str(&format!("#EXTINF:{}\n", segment.duration));
        playlist.push_str(&format!(
            "http://localhost:8000/segments/{}_{}.ts\n",
            variant_pid, segment.position
        ));
    }

    playlist.push_str("#EXT-X-ENDLIST");

    let content_type = ContentType::new("application", "vnd.apple.mpegurl");

    Result::Ok(Custom(content_type, playlist))
}

#[get("/videos/<file_name>")]
async fn video(file_name: &str) -> Result<Custom<String>, NotFound<String>> {
    let regex = Regex::new(r"([a-zA-Z0-9]{10}).m3u8")
        .map_err(|_| NotFound("Invalid regex.".to_string()))?;

    let capture_groups = match regex.captures(file_name) {
        Some(capture_groups) => capture_groups,
        None => return Result::Err(NotFound(String::from("Invalid filename."))),
    };

    let video_pis = match capture_groups.get(1) {
        Some(video_pis) => video_pis.as_str(),
        None => return Result::Err(NotFound(String::from("No identifier in filename."))),
    };

    let video = Video::find_by_public_id(video_pis)
        .map_err(|_| NotFound("Video not found.".to_string()))?;

    let variants = Variant::by_video(&video);

    if variants.is_empty() {
        return Result::Err(NotFound("Video has no variants.".to_string()));
    }

    let mut playlist = String::from("#EXTM3U\n");

    for variant in variants.into_iter() {
        playlist.push_str(&format!(
            "#EXT-X-STREAM-INF:BANDWIDTH={},RESOLUTION={}x{},CODECS=\"avc1.42e00a,mp4a.40.2\"\n",
            variant.bitrate, variant.width, variant.height
        ));
        playlist.push_str(&format!(
            "http://localhost:8000/variants/{}.m3u8\n",
            variant.public_id
        ));
    }

    let content_type = ContentType::new("application", "vnd.apple.mpegurl");

    Result::Ok(Custom(content_type, playlist))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![segment, variant, video])
}
