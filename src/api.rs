use actix_web::{web, get, post, HttpResponse};
use serde_derive::{Deserialize, Serialize};

use crate::CONFIG;
use crate::db;
use crate::db::models::Posts;
use hmac::{Hmac, NewMac};
use jwt_simple::prelude::*;

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Info {
    pub blog_info: BlogInfo,
    pub account_info: AccountInfo,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct BlogInfo {
    pub blog_name: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AccountInfo {
    pub success: bool,
    pub pk: Option<i32>,
    pub username: Option<String>,
    pub email: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RegisterForm {
    pub username: String,
    pub pass: String, // SHA256 hashed
    pub email: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct LoginForm {
    pub username: String,
    pub pass: String,
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

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AccountResult {
    pub success: bool,
    pub error_msg: Option<AccountError>,
    pub token: Option<String>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AccountToken {
    pub is_logined: bool,
    pub pk: i32,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TokenRequest {
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GetPosts {
    pub start: i64,
    pub limit: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreatePost {
    pub token: String,
    pub title: String,
    pub body: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Author {
    pub name: String,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PostsResponse {
    pub success: bool,
    pub error_msg: Option<String>,
    pub body: Option<Vec<(Posts, Author)>>,
}

// GET /api/posts
#[get("/api/posts")]
pub async fn posts(web::Query(form): web::Query<GetPosts>) -> HttpResponse {
    let body = if let Ok(res) = db::get_posts(form.start, form.limit) {
        Some(res)
    } else {
        None
    };
    let response = PostsResponse {
        success: body.is_some(),
        error_msg: None, // TODO
        body,
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}

// POST /api/create_post
#[post("/api/create_post")]
pub async fn create_post(form: web::Json<CreatePost>) -> HttpResponse {
    let config = CONFIG.clone();
    let key = HS256Key::from_bytes(config.secret().token_secret().as_bytes());
    let account = if let Ok(auth) = key.verify_token::<AccountToken>(&form.token, None) {
        if let Ok(accounts) = db::find_user(auth.custom.pk) {
            Some(accounts)
        } else {
            None
        }
    } else {
        None
    };
    let response = 
    if let Some(auth) = account {
        if let Ok(_) = db::create_post(auth.id, &form.title, &form.body) {
            PostsResponse {
                success: true,
                error_msg: None, // TODO
                body: None,
            }
        } else {
            PostsResponse {
                success: true,
                error_msg: None, // TODO
                body: None,
            }
        }
    } else {
        PostsResponse {
            success: false,
            error_msg: None, // TODO
            body: None,
        }
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(response)
}

// GET /api/info
#[get("/api/info")]
pub async fn info(web::Query(form): web::Query<TokenRequest>) -> HttpResponse {
    let config = CONFIG.clone();
    let key = HS256Key::from_bytes(config.secret().token_secret().as_bytes());
    let account = if let Ok(auth) = key.verify_token::<AccountToken>(&form.token, None) {
        if let Ok(accounts) = db::find_user(auth.custom.pk) {
            Some(accounts)
        } else {
            None
        }
    } else {
        None
    };
    let account_info = if let Some(auth) = account {
        AccountInfo {
            success: true,
            pk: Some(auth.id),
            username: Some(auth.username),
            email: Some(auth.email),
        }
    } else {
        AccountInfo {
            success: false,
            pk: None,
            username: None,
            email: None,
        }
    };
    let blog_info = BlogInfo {
        blog_name: config.general().blog_name(),
    };
    let info = Info {
        blog_info,
        account_info,
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(info)
}


// POST /api/login
#[post("/api/login")]
pub async fn login(form: web::Json<LoginForm>) -> HttpResponse {
    let config = CONFIG.clone();
    let result = db::get_login(&form.username, &form.pass);
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
    let result = db::create_account(&form.username, &form.pass, &form.email);
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
