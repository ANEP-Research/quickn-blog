use crate::db::schema::*;
use std::time::SystemTime;
use serde_derive::{Deserialize, Serialize};

#[derive(Queryable)]
pub struct Accounts {
    pub id: i32,
    pub username: String,
    pub pass: String,
    pub email: String,
}

#[derive(Queryable, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Posts {
    pub id: i32,
    pub author: i32,
    pub title: String,
    pub body: String,
    pub created: SystemTime,
    pub modified: Option<SystemTime>,
}

#[derive(Queryable, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Comments {
    pub id: i32,
    pub author: i32,
    pub post: i32,
    pub title: String,
    pub body: String,
    pub created: SystemTime,
    pub modified: Option<SystemTime>,
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub pass: &'a str,
    pub email: &'a str
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub author: i32,
    pub title: &'a str,
    pub body: &'a str,
    pub created: SystemTime,
}

#[derive(Insertable)]
#[table_name="comments"]
pub struct NewComment<'a> {
    pub author: i32,
    pub post: i32,
    pub title: &'a str,
    pub body: &'a str,
    pub created: SystemTime,
}