use crate::{models::Rustacean, schema::rustaceans};
use diesel::prelude::*;
pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
    }
}
