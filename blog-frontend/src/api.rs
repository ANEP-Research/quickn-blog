use serde_derive::{Deserialize, Serialize};
use yew::utils::host;
use crate::errors::*;

// GET /api/info
#[derive(Deserialize, Serialize, Debug)]
pub struct BlogInfo {
    pub blog_name: String,
}

impl BlogInfo {
    pub async fn new() -> Result<Self, FetchError> {
        let res = reqwest::get(&format!("http://{}/api/info", host().unwrap()))
            .await
            .unwrap();
        let text = res.text().await.unwrap();
        let info: BlogInfo = serde_json::from_str(&text).unwrap();
        Ok(info)
    }
}
