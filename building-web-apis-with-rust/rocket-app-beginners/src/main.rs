/// Rocket documentation: https://rocket.rs/guide/v0.5/
/// Getting Started: https://rocket.rs/guide/v0.5/getting-started/#getting-started

#[macro_use] extern crate rocket;

// endpoint
// can be called on terminal with `curl 127.0.0.1:8000``
#[get("/")]
fn index() -> &'static str {
    "Hello, world from Rocket!\n"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
