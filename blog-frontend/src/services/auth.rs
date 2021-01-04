use yew::utils::host;
use crate::services::CookieService;
use crate::api::*;
use crate::errors::FetchError;
use wasm_bindgen::prelude::*;

pub struct AuthService {
    cookie: CookieService,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            cookie: CookieService::new(),
        }
    }

    pub async fn get_user(&self) -> Result<UserInfo, FetchError> {
        if let Ok(token) = self.cookie.get("token") {
            let json = TokenRequest { token };
            let client = reqwest::Client::new();
            let res_packed = client
                .post(&format!("http://{}/api/user_info", host().unwrap()))
                .json(&json)
                .send()
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
        } else {
            Err(FetchError::from(JsValue::from_str("Invalid token")))
        }
    }
}