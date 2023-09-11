use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ArticleCmd {
    pub title: String,
    pub body: String,
    pub tags: Vec<Uuid>,
}

#[derive(Serialize)]
pub struct ArticleVo {
    pub title: String,

    pub body: String,

    pub tags: Vec<TagVo>,
}

#[derive(Serialize)]
pub struct TagVo {
    pub name: String,
}

#[derive(Serialize)]
pub struct ArticleSummary {
    pub id: Uuid,

    pub title: String,

    pub release_date: String,
}
