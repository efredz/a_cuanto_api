use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::local_model::property;
use crate::local_model::property::Property;
use crate::schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_all_properties() {
    use schema::property::dsl::*;

    let connection = establish_connection();

    let properties = property.load::<Property>(&connection)
        .expect("Error loading properties");
}