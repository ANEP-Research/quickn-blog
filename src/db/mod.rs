pub mod models;
pub mod schema;

use crate::middlewares::postgresql::establish_connection;
use crate::api::{AccountError, Author};
use diesel::prelude::*;
use schema::*;
use models::*;
use sha3::{Digest, Sha3_256};
use std::time::SystemTime;

pub fn create_account<'a>(username: &'a str, pass: &'a str, email: &'a str) -> QueryResult<Accounts> {
    let db = establish_connection();
    let mut hasher = Sha3_256::new();
    hasher.update(pass.as_bytes());
    let pass_hashed = hasher.finalize();
    let new_account = NewAccount { username, pass: &hex::encode(pass_hashed), email };
    diesel::insert_into(accounts::table)
        .values(&new_account)
        .get_result(&db)
}

pub fn get_login<'a>(username: &'a str, pass: &'a str) -> QueryResult<(Option<AccountError>, i32)> {
    let db = establish_connection();
    let mut hasher = Sha3_256::new();
    hasher.update(pass.as_bytes());
    let pass_hashed = hasher.finalize();
    let mut items = accounts::table.filter(accounts::dsl::username.eq(username)).load::<Accounts>(&db)?;
    if let Some(user) = items.pop() {
        if hex::encode(pass_hashed) == user.pass {
            Ok((None, user.id))
        } else {
            Ok((Some(AccountError::PassNotMatched), -1))
        }
    } else {
        Ok((Some(AccountError::UserNotExists), -1))
    }
}

pub fn find_user<'a>(pk: i32) -> QueryResult<Accounts> {
    let db = establish_connection();
    accounts::table.find(pk).first(&db)
}

pub fn create_post<'a>(user: i32, title: &'a str, body: &'a str) -> QueryResult<Posts> {
    let db = establish_connection();
    let now = SystemTime::now();
    let new_post = NewPost { author: user, title, body, created: now };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&db)
}

pub fn get_posts<'a>(start: i64, take: i64) -> QueryResult<Vec<(Posts, Author)>> {
    let db = establish_connection();
    if let Ok(posts) = posts::table
        .limit(take)
        .offset(start)
        .load::<Posts>(&db) {
            Ok(
                posts.iter().map(|post| 
                    if let Ok(user) = find_user(post.author) {
                        let author = Author {
                            name: user.username,
                        };
                        (post.clone(), author)
                    } else {
                        (post.clone(), Author::default())
                    }
                ).collect()
            )
        } else {
            Err(diesel::result::Error::NotFound)
        }
}


