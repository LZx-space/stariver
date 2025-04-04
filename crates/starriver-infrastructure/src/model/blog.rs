use sea_orm::FromQueryResult;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, FromQueryResult)]
pub struct BlogPreview {
    pub id: Uuid,

    pub title: String,

    pub create_at: OffsetDateTime,
}
