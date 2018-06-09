use rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[test]
fn index_test() {
    let instance = rocket::ignite()
        .mount("/", routes![index]);
    let client = rocket::local::Client::new(instance)
        .unwrap();
    let mut res = client.get("/").dispatch();
    assert_eq!(res.body_string()
        , Some("Hello, world!".into()));
}