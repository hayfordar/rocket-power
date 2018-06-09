use rocket_contrib::{Json};
use rocket::http::{Status, ContentType};

#[get("/", format = "application/json")]
fn index() -> Json {
    Json(json!({
        "status" : "good"
    }))
}

#[test]
fn index_test() {
    let instance = rocket::ignite()
        .mount("/", routes![index]);
    let client = rocket::local::Client::new(instance)
        .unwrap();
    
    let mut res = client.get("/")
        .header(ContentType::JSON)
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    
    let body = res.body()
        .unwrap()
        .into_string()
        .unwrap();

    assert!(body.contains("good"));
    
}