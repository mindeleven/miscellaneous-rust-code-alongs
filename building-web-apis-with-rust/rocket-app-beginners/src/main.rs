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

use auth::BasicAuth;

// importing json macro
use rocket::{
    response::status, 
    serde::json:: {
        json, 
        Value
    }
};

// no auth -> curl 127.0.0.1:8000/rustaceans
// with auth ->
// curl 127.0.0.1:8000/rustaceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([
        { "id": 1,  "name": "John Doe" },
        { "id": 1,  "name": "Jane Doe" }
    ])
}

// curl 127.0.0.1:8000/rustaceans/123
#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32) -> Value {
    json!([
        { "id": id,  "name": "John Doe", "email": "john.doe@example.com" }
    ])
}
 // curl 127.0.0.1:8000/rustaceans/ -X POST -H 'Content-type: application/json'
 // curl 127.0.0.1:8000/rustaceans/ -X POST -H 'Content-type: application/json' -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
 #[post("/rustaceans", format="json")]
fn create_rustaceans(_auth: BasicAuth) -> Value {
    json!([
        { "id": 3,  "name": "John Doe", "email": "john.doe@example.com" }
    ])
}

// curl 127.0.0.1:8000/rustaceans/12 -X PUT -H 'Content-type: application/json'
#[put("/rustaceans/<id>", format="json")]
fn update_rustaceans(id: i32, _auth: BasicAuth) -> Value {
    json!([
        { "id": id,  "name": "John Doe", "email": "john.doe@example.com" }
    ])
}

// curl 127.0.0.1:8000/rustaceans/12 -X DELETE -I
// the -I parameter varifies that there is no content
#[allow(unused_variables)]
#[delete("/rustaceans/<id>")]
fn delete_rustaceans(id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
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
            not_authorized
        ])
}