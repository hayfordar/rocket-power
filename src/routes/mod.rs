use rocket;

pub mod index;

pub fn all_routes() -> Vec<rocket::Route> {
    routes![index::index]
}