#[macro_use] extern crate rocket;

mod models;
mod routes;

use routes::{hello::say_hello , home::hello,register::register};

#[launch]
fn rocket()->_{
    rocket::build().mount("/", routes![say_hello,hello,register])
}