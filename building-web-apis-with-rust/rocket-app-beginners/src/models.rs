use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String
}