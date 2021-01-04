use crate::db::schema::accounts;

#[derive(Queryable)]
pub struct Accounts {
    pub id: i32,
    pub username: String,
    pub pass: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name="accounts"]
pub struct NewAccount<'a> {
    pub username: &'a str,
    pub pass: &'a str,
    pub email: &'a str
}