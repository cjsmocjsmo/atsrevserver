use actix_web::{get, post, web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};
// use polodb_core::{Database, bson::de};
use polodb_core::Database;
use log::{error, info};

pub mod server_functions;
pub mod atstypes;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/insert_est")]
async fn insert_est(info: web::Json<atstypes::EstInInfo>) -> impl Responder {
    let acctid = server_functions::check_for_existing_account(info.email.clone());
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("Could not open db file");
    let estscoll = db.collection("estimates");
    let est = atstypes::EstOutInfo {
        acctid: acctid.clone(),
        name: info.name.clone(),
        addr: info.addr.clone(),
        city: info.city.clone(),
        phone: info.phone.clone(),
        email: info.email.clone(),
        reqservdate: info.reqservdate.clone(),
        comment: info.comment.clone(),
    };

    info!(target: "atsrevserver", "insert_est est: {:?}", est);

    estscoll.insert_one(est).expect("unable to insert revs");

    HttpResponse::Ok().body("Insert estimate is complete")
}

#[get("/allests")]
async fn allests() -> impl Responder {
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("Could not open db file");
    let estscoll = db.collection("estimates");
    let ests = estscoll.find(None).expect("could not find ests");

    let mut est_vec = Vec::new();
    for e in ests {
        let e_result = e;
        let res = match e_result {
            Ok(e) => e,
            Err(e) => {
                error!(target: "atsrevserver", "allests error: {:?}", e);
                continue;
            }
        };
        
        est_vec.push(res);
    }

    let aests = serde_json::to_string(&est_vec).expect("unable to serialize ests");
    info!(target: "atsrevserver", "allests aests: {:?}", aests);



    HttpResponse::Ok().json(aests)
}

#[post("/insert_rev")]
async fn insert_review(info: web::Json<atstypes::RevInInfo>) -> impl Responder {
    let acctid = server_functions::check_for_existing_account(info.email.clone());
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("Could not open db file");
    let revscoll = db.collection("reviews");
    let rev = atstypes::RevOutInfo {
        acctid: acctid.clone(),
        name: info.name.clone(),
        email: info.email.clone(),
        stars: info.stars.clone(),
        review: info.review.clone(),
    };
    // info!(target: "atsrevserver", "insert_review boo: {:?}", boo);

    revscoll.insert_one(rev).expect("unable to insert revs");

    HttpResponse::Ok().body("ReviewInserted")
}


#[get("allrevs")]
async fn allrevs() -> impl Responder {
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("could not open db file");
    let revscoll = db.collection::<atstypes::RevOutInfo>("reviews");
    let revs = revscoll.find(None).expect("could not find reives");

    let mut rev_vec = Vec::new();
    for r in revs {
        let r_result = r;
        let res = match r_result {
            Ok(r) => r,
            Err(e) => {
                error!(target: "atsrevserver", "allrevs error: {:?}", e);
                continue;
            }
        };
        
        rev_vec.push(res);
    }

    let arevs = serde_json::to_string(&rev_vec).expect("unable to serialize revs");
    // info!(target: "atsrevserver", "allrevs arevs: {:?}", arevs);
    HttpResponse::Ok().json(arevs)
}