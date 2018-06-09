#![feature(extern_prelude)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate diesel;
extern crate serde; 

mod routes;
mod store;
mod database;

static MYSQL_CONNECTION_STRING: &str = env!("MYSQL_CONNECTION_STRING");

fn main() {
    let state = store::Store{ database::spawn(MYSQL_CONNECTION_STRING) }

    rocket::ignite()
        .mount("/", routes::all_routes())
        .manage(state)
        .launch();
}
