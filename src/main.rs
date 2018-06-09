#![feature(extern_prelude)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use] extern crate rocket_contrib;

mod routes;

fn main() {
    rocket::ignite()
        .mount("/", routes::all_routes())
        .launch();
}
