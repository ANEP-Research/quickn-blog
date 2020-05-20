use actix_web::{get, HttpResponse};
use serde_derive::Serialize;

use crate::CONFIG;

#[derive(Serialize, Debug)]
pub struct BlogInfo {
    blog_name: String,
}

#[get("/api/info")]
pub async fn info() -> HttpResponse {
    let config = CONFIG.clone();
    let blog_info = BlogInfo {
        blog_name: config.general().blog_name(),
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(blog_info)
}
