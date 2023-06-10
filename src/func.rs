use actix_web::{get, post, web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};
use polodb_core::Database;


pub mod server_functions;
pub mod atstypes;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/allests")]
async fn allests() -> impl Responder {
    HttpResponse::Ok().body("Hello allests")
}

#[get("/insert_rev")]
async fn insert_review(info: web::Json<atstypes::QInfo>) -> impl Responder {
    println!("{:?}", info);
    let acctid = server_functions::create_account(info.email.clone());
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").unwrap();
    let revscoll = db.collection("reviews");
    revscoll
        .insert_one(atstypes::IInfo {
            acctid: acctid.clone(),
            name: info.name.clone(),
            email: info.email.clone(),
            stars: info.stars.clone(),
            review: info.review.clone(),
        })
        .unwrap();

    HttpResponse::Ok().body("ReviewInserted")
}

#[get("allrevs")]
async fn allrevs() -> impl Responder {
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").unwrap();
    let revscoll = db.collection::<atstypes::IInfo>("reviews");
    let revs = revscoll.find(None).unwrap();

    let mut rev_vec = Vec::new();
    for r in revs {
        let foo = format!("{:?}", r);
        rev_vec.push(foo);
    };

    let arevs = serde_json::to_string(&rev_vec).unwrap();

    HttpResponse::Ok().json(arevs)
}