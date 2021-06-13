#[derive(Debug, Queryable)]
pub struct Video {
    pub id: i64,
    pub title: Option<String>,
    pub published: Option<bool>,
    pub status: Option<i32>,
    pub source: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub public_id: String,
    pub user_id: i64,
}
