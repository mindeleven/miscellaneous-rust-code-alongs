use crate::{
    models::Rustacean, 
    schema::rustaceans
};
use diesel::prelude::*;
pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
    }

    pub fn find_multiple(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table
            .order(rustaceans::id.desc())
            .limit(limit)
            .load::<Rustacean>(c)
    }
}
