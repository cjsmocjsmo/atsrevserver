use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use polodb_core::Database;


pub mod server_functions;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/allests")]
async fn allests() -> impl Responder {
    HttpResponse::Ok().body("Hello allests")
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct QInfo {
    name: String,
    email: String,
    stars: String,
    review: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct IInfo {
    acctid: String,
    name: String,
    email: String,
    stars: String,
    review: String,
}

#[get("/insert_rev")]
async fn insert_review(info: web::Query<QInfo>) -> impl Responder {
    println!("{:?}", info);
    let acctid = server_functions::create_account(info.email.clone());
    let db = Database::open_file("ats.db").unwrap();
    let revscoll = db.collection("reviews");
    revscoll
        .insert_one(IInfo {
            acctid: acctid.clone(),
            name: info.name.clone(),
            email: info.email.clone(),
            stars: info.stars.clone(),
            review: info.review.clone(),
        })
        .unwrap();

    HttpResponse::Ok().body("Hello allests")
}