#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod repository;
mod local_model;
mod schema;

use rocket_contrib::json::Json;
use crate::local_model::property::Property;

#[get("/")]
fn index() -> Json<Vec<Property>> {
    let properties = repository::property_repository::get_all_properties();
    return Json(properties);
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
