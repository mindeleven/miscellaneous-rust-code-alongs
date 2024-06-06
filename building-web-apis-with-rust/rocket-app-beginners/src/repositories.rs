use crate::{
    models::{NewRustacean, Rustacean}, 
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

    pub fn create(c: &mut SqliteConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .execute(c)?;
        
        let last_id = Self::last_inserted_id(c)?;

        Self::find(c, last_id)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        rustaceans::table.select(rustaceans::id) 
            .order(rustaceans::id.desc())
            .first(c)
    }
}
