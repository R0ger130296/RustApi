#![feature(plugin, const_fn, decl_macro,)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;

extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate rocket_cors;


use rocket_cors::{AllowedOrigins};

use dotenv::dotenv;
use std::env;
use routes::*;
mod db;
mod models;
mod routes;
mod schema;


fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);

    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]);
  
    let cors = rocket_cors::CorsOptions {
      allowed_origins,
      allow_credentials: true,
      ..Default::default()
    }
    .to_cors()
    // panic if there was an error
    .expect("error creating CORS fairing");
    rocket::ignite()
        .manage(pool)
        .mount(
            "/",
            routes![index, new, show, delete, update],
        ).attach(cors)
}

fn main() {
    rocket().launch();
}

