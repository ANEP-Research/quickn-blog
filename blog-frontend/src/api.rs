use crate::errors::*;
use serde_derive::{Deserialize, Serialize};
use std::fmt;
use std::future::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::utils::host;

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        FetchError { err: value }
    }
}

#[derive(Clone, PartialEq)]
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

pub fn send_future<COMP: Component, F>(link: ComponentLink<COMP>, future: F)
where
    F: Future<Output = COMP::Message> + 'static,
{
    spawn_local(async move {
        link.send_message(future.await);
    });
}

// GET /api/info
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
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

// POST /api/register
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RegisterForm {
    pub username: String,
    pub pass: String, // SHA256 hashed
    pub email: String,
}

impl RegisterForm {
    pub async fn send(&self) -> Result<AccountResult, FetchError> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("http://{}/api/register", host().unwrap()))
            .json(&self)
            .send()
            .await
            .unwrap();
        let text = res.text().await.unwrap();
        let info: AccountResult = serde_json::from_str(&text).unwrap();
        Ok(info)
    }
}

// POST /api/login
#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoginForm {
    pub username: String,
    pub pass: String,
}

impl LoginForm {
    pub async fn send(&self) -> Result<AccountResult, FetchError> {
        let client = reqwest::Client::new();
        let res = client
            .post(&format!("http://{}/api/login", host().unwrap()))
            .json(&self)
            .send()
            .await
            .unwrap();
        let text = res.text().await.unwrap();
        let info: AccountResult = serde_json::from_str(&text).unwrap();
        Ok(info)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum AccountError {
    AlreadyExistsUsername,
    AlreadyExistsEmail,
    NotValidEmail,
    PasswordTooWeak,
    UserNotExists,
    PassNotMatched,
    NetworkError(String),
    DbError(String)
}

impl fmt::Display for AccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountError::AlreadyExistsUsername => write!(f, "Username already exists."),
            AccountError::AlreadyExistsEmail => write!(f, "E-mail already exists."),
            AccountError::NotValidEmail => {
                write!(f, "Your E-mail is not valid. Re-check about it.")
            }
            AccountError::PasswordTooWeak => write!(
                f,
                "Your password is too short. Please set at least length 8."
            ),
            AccountError::NetworkError(s) => write!(f, "Some network error occurs. {}", s),
            AccountError::DbError(s) => write!(f, "Some database error occurs. {}", s),
            AccountError::UserNotExists => write!(f, "Your user not exists."),
            AccountError::PassNotMatched => write!(f, "Your password is not matched."),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AccountResult {
    pub success: bool,
    pub error_msg: Option<AccountError>,
    pub token: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct UserInfo {
    pub success: bool,
    pub pk: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TokenRequest {
    pub token: String,
}
