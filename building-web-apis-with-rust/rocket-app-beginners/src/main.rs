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
mod repositories;

use auth::BasicAuth;

// importing json macro
use rocket::{
    http::Status, response::status::{self, Custom}, serde::json:: {
        json,
        Json,
        Value
    }
};
use rocket_sync_db_pools::database;
use crate::models::{
    Rustacean, 
    NewRustacean
};
use repositories::RustaceanRepository;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

// no auth -> curl 127.0.0.1:8000/rustaceans
// with auth ->
// curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    // db is a connection pool
    // getting a connection from the pool with db.run()
    // run will accept connection in callback and run async
    db.run(|c| {
        RustaceanRepository::find_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

// curl 127.0.0.1:8000/rustaceans/3 -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans/<id>")]
async fn view_rustaceans(id: i32, _auth: BasicAuth, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::find(c, id)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}
 // curl 127.0.0.1:8000/rustaceans/ -X POST -H 'Content-type: application/json'
 // curl 127.0.0.1:8000/rustaceans/ -X POST -H 'Content-type: application/json' -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -d '{"name": "Mike", "email": "mike@foo.xrz"}'
 #[post("/rustaceans", format="json", data="<new_rustacean>")]
async fn create_rustaceans(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        RustaceanRepository::create(c, new_rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

// curl 127.0.0.1:8000/rustaceans/3 -X PUT -H 'Content-type: application/json' -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ==' -d '{"name": "Bob", "email": "Bob@blacklodege.xrz"}'
#[put("/rustaceans/<id>", format="json", data="<rustacean>")]
async fn update_rustaceans(id: i32, db: DbConn, _auth: BasicAuth, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::save(c, id, rustacean.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

// curl 127.0.0.1:8000/rustaceans/1 -X DELETE -I -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
// the -I parameter varifies that there is no content
#[allow(unused_variables)]
#[delete("/rustaceans/<id>")]
async fn delete_rustaceans(id: i32, db: DbConn, _auth: BasicAuth) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
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
