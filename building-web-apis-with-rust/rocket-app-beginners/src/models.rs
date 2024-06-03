use diesel::{deserialize::Queryable, prelude::Insertable, AsChangeset};
use rocket::serde::{Deserialize, Serialize};
use crate::schema::rustaceans;

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}