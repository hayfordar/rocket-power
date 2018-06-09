#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

mod routes;

fn main() {
    rocket::ignite()
        .mount("/", routes::all_routes())
        .launch();
}
