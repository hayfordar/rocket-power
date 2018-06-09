use rocket_contrib::{Json};

#[get("/rest/api/index")]
fn index() -> Json {
    Json(json!({
        "status" : "ok"
    }))
}

#[test]
fn index_test() {
    let instance = rocket::ignite()
        .mount("/", routes![index]);
    let client = rocket::local::Client::new(instance)
        .unwrap();
    
    let mut res = client.get("/rest/api/index")
        .header(rocket::http::ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), rocket::http::Status::Ok);
    
    let body = res.body()
        .unwrap()
        .into_string()
        .unwrap();
    assert!(body.contains("ok"));
}