pub mod models;
pub mod schema;

use crate::middlewares::postgresql::establish_connection;
use crate::api::AccountError;
use diesel::prelude::*;
use schema::accounts;
use models::{Accounts, NewAccount};
use sha3::{Digest, Sha3_256};

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