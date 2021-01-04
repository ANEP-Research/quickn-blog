use actix_web::{web, get, post, HttpResponse};
use serde_derive::{Deserialize, Serialize};

use crate::CONFIG;
use crate::db::{create_account, get_login, find_user};
use hmac::{Hmac, NewMac};
use jwt_simple::prelude::*;

#[derive(Serialize, Debug)]
pub struct BlogInfo {
    pub blog_name: String,
}

#[derive(Serialize, Debug)]
pub struct UserInfo {
    pub success: bool,
    pub pk: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    pub username: String,
    pub pass: String, // SHA256 hashed
    pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub username: String,
    pub pass: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Debug, Clone)]
pub struct AccountResult {
    pub success: bool,
    pub error_msg: Option<AccountError>,
    pub token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AccountToken {
    pub is_logined: bool,
    pub pk: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenRequest {
    pub token: String,
}

// GET /api/info
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

// POST /api/user_info
#[post("/api/user_info")]
pub async fn user_info(form: web::Json<TokenRequest>) -> HttpResponse {
    let config = CONFIG.clone();
    let key = HS256Key::from_bytes(config.secret().token_secret().as_bytes());
    let account = if let Ok(auth) = key.verify_token::<AccountToken>(&form.token, None) {
        if let Ok(accounts) = find_user(auth.custom.pk) {
            Some(accounts)
        } else {
            None
        }
    } else {
        None
    };
    let account_info = if let Some(auth) = account {
        UserInfo {
            success: true,
            pk: Some(auth.id),
            username: Some(auth.username),
            email: Some(auth.email),
        }
    } else {
        UserInfo {
            success: false,
            pk: None,
            username: None,
            email: None,
        }
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(account_info)
}


// POST /api/login
#[post("/api/login")]
pub async fn login(form: web::Json<LoginForm>) -> HttpResponse {
    let config = CONFIG.clone();
    let result = get_login(&form.username, &form.pass);
    let json = if let Err(e) = result {
        AccountResult {
            success: false,
            error_msg: Some(AccountError::DbError(format!("{:?}", e))),
            token: None,
        }
    } else {
        let s = result.unwrap();
        AccountResult {
            success: if s.1 == -1 { false } else { true },
            error_msg: s.0,
            token: if s.1 == -1 { None } else { 
                let key = HS256Key::from_bytes(config.secret().token_secret().as_bytes());
                let auth = AccountToken {
                    is_logined: true,
                    pk: s.1,
                };
                let claims = Claims::with_custom_claims(auth, Duration::from_days(1));
                let token = key.authenticate(claims).unwrap();
                Some(token)
            },
        }
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json)
}

// POST /api/register
#[post("/api/register")]
pub async fn register(form: web::Json<RegisterForm>) -> HttpResponse {
    let result = create_account(&form.username, &form.pass, &form.email);
    let json = if let Err(e) = result {
        AccountResult {
            success: false,
            error_msg: Some(AccountError::DbError(format!("{:?}", e))),
            token: None,
        }
    } else {
        AccountResult {
            success: true,
            error_msg: None,
            token: None,
        }
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(json)
}
