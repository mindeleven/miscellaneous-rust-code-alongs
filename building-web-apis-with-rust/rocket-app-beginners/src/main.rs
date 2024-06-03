/// Rocket documentation: https://rocket.rs/guide/v0.5/
/// Getting Started: https://rocket.rs/guide/v0.5/getting-started/#getting-started
/// 
/// Endpoints:
/// GET list existing
/// GET show single
/// POST create new
/// PUT update existing
/// DELETE delete existing

#[macro_use] extern crate rocket;

mod auth;
mod schema;
mod models;

use auth::BasicAuth;

use diesel::prelude::*;
// importing json macro
use rocket::{
    response::status, 
    serde::json:: {
        json,
        Json,
        Value
    }
};
use rocket_sync_db_pools::database;
use schema::rustaceans;
use crate::models::{
    Rustacean, 
    NewRustacean
};

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

// no auth -> curl 127.0.0.1:8000/rustaceans
// with auth ->
// curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    // db is a connection pool
    // getting a connection from the pool with db.run()
    // run will accept connection in callback and run async
    db.run(|c| {
        let rustaceans = rustaceans::table
            .order(rustaceans::id.desc())
            .limit(1000)
            .load::<Rustacean>(c)
            .expect("Database error");
        json!(rustaceans)
    }).await
    /*
    json!([
        { "id": 1,  "name": "John Doe" },
        { "id": 1,  "name": "Jane Doe" }
    ])
    */
}

// curl 127.0.0.1:8000/rustaceans/1 -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans/<id>")]
async fn view_rustaceans(id: i32, _auth: BasicAuth, db: DbConn) -> Value {
    db.run(move |c| {
        let rustaceans = rustaceans::table
            .find(id)
            .get_result::<Rustacean>(c)
            .expect("Database error when selecting rustacean");

        json!(rustaceans)
    }).await

    /* json!([
        { "id": id,  "name": "John Doe", "email": "john.doe@example.com" }
    ]) */
}
 // curl 127.0.0.1:8000/rustaceans/ -X POST -H 'Content-type: application/json'
 // curl 127.0.0.1:8000/rustaceans/ -X POST -H 'Content-type: application/json' -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -d '{"name": "Jane", "email": "jane@foo.xrz"}'
 #[post("/rustaceans", format="json", data="<new_rustacean>")]
async fn create_rustaceans(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Database error when inserting");
        json!(result)
    }).await

    /* json!([
        { "id": 3,  "name": "John Doe", "email": "john.doe@example.com" }
    ]) */
}

// curl 127.0.0.1:8000/rustaceans/3 -X PUT -H 'Content-type: application/json' -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -d '{"name": "Jane Z. Russel", "email": "janeZ@russel.xrz"}'
#[put("/rustaceans/<id>", format="json", data="<rustacean>")]
async fn update_rustaceans(id: i32, db: DbConn, _auth: BasicAuth, rustacean: Json<Rustacean>) -> Value {
    db.run(move |c| {
        let result = diesel::update(
                rustaceans::table.find(id)
            ).set((
                rustaceans::name.eq(rustacean.name.to_owned()),
                rustaceans::email.eq(rustacean.email.to_owned()),
            ))
            .execute(c)
            .expect("Database error when updating rustacean");
        json!(result)
    }).await
    
    /* json!([
        { "id": id,  "name": "John Doe", "email": "john.doe@example.com" }
    ]) */
}

// curl 127.0.0.1:8000/rustaceans/2 -X DELETE -I -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
// the -I parameter varifies that there is no content
#[allow(unused_variables)]
#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id: i32, db: DbConn, _auth: BasicAuth) -> status::NoContent {
    db.run(move |c| {
        diesel::delete(
                rustaceans::table.find(id))
            .execute(c)
            .expect("Database error when deleting rustacean");
        status::NoContent
    }).await

    // status::NoContent
}

// endpoint
// can be called on terminal with `curl 127.0.0.1:8000``
/* #[get("/")]
fn index() -> Value {
    json!("Hello, world from JSON!")
} */

// default error catcher
// `curl 127.0.0.1:8000/ -I` or `curl 127.0.0.1:8000/` 
#[allow(dead_code)]
#[catch(404)]
fn not_found() -> Value {
    json!("Not found!")
}

// HTTP status code 401, “Unauthorized,”
#[allow(dead_code)]
#[catch(401)]
fn not_authorized() -> Value {
    json!("The request requires user authentication!")
}

// HTTP status code 401, “Unauthorized,”
#[allow(dead_code)]
#[catch(422)]
fn unprocessable_entity() -> Value {
    json!("The server can't process your request!")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            get_rustaceans,
            view_rustaceans,
            update_rustaceans,
            create_rustaceans,
            delete_rustaceans
        ])
        .register("/", catchers![
            not_found,
            not_authorized,
            unprocessable_entity
        ])
        .attach(DbConn::fairing()) // attaching a fairing before launching
}
