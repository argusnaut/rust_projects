#[macro_use]
extern crate rocket;

mod enums;
mod routes;
mod services;

use routes::demon::get_demon;
use routes::demon::create_demon;

#[get("/")]
fn api() -> &'static str {
    "Hello, welcome to the api!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![api, get_demon, create_demon])
}
