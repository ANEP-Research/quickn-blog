use crate::api::*;
use crate::errors::FetchError;
use crate::services::CookieService;
use wasm_bindgen::prelude::*;
use yew::utils::host;

pub struct AuthService {
    cookie: CookieService,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            cookie: CookieService::new(),
        }
    }

    pub async fn get(&self) -> Result<Info, FetchError> {
        let token = self.cookie.get("token").unwrap_or(String::new());
        let client = reqwest::Client::new();
        let res_packed = reqwest::get(&format!(
            "http://{}/api/info?token={}",
            host().unwrap(),
            token
        ))
        .await;
        if let Ok(res) = res_packed {
            let text = res.text().await.unwrap();
            let info_packed = serde_json::from_str(&text);
            if let Ok(info) = info_packed {
                Ok(info)
            } else {
                if let Err(e) = info_packed {
                    Err(FetchError::from(JsValue::from_str(&format!("{}", e))))
                } else {
                    Err(FetchError::from(JsValue::from_str("Unknown error")))
                }
            }
        } else {
            if let Err(e) = res_packed {
                Err(FetchError::from(JsValue::from_str(&format!("{}", e))))
            } else {
                Err(FetchError::from(JsValue::from_str("Unknown error")))
            }
        }
    }
}
