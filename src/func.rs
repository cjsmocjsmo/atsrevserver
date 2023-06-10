use actix_web::{get, post, web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};
use polodb_core::{Database, bson::de};
use log::{error, info, debug};

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

#[post("/insert_rev")]
async fn insert_review(info: web::Json<atstypes::QInfo>) -> impl Responder {
    info!(target: "atsrevserver", "insert_review: {:?}", info);
    let acctid = server_functions::create_account(info.email.clone());
    info!(target: "atsrevserver", "insert_review: acctid: {:?}", acctid);
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("Could not open db file");
    let revscoll = db.collection("reviews");
    let boo = atstypes::IInfo {
        acctid: acctid.clone(),
        name: info.name.clone(),
        email: info.email.clone(),
        stars: info.stars.clone(),
        review: info.review.clone(),
    };
    info!(target: "atsrevserver", "insert_review: boo: {:?}", boo);

    revscoll.insert_one(boo).unwrap();

    HttpResponse::Ok().body("ReviewInserted")
}

#[get("allrevs")]
async fn allrevs() -> impl Responder {
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").unwrap();
    let revscoll = db.collection::<atstypes::IInfo>("reviews");
    let revs = revscoll.find(None).expect("could not find reives");

    

    let mut rev_vec = Vec::new();
    for r in revs {
        info!(target: "atsrevserver", "allrevs: {:?}", r);
        let foo = format!("{:?}", r);
        info!(target: "atsrevserver", "llrevs: {:?}", foo);
        rev_vec.push(foo);
    };

    let arevs = serde_json::to_string(&rev_vec).unwrap();

    HttpResponse::Ok().json(arevs)
}