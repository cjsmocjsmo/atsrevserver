use actix_web::{get, post, web, HttpResponse, Responder};
// use serde::{Deserialize, Serialize};
use chrono::Local;
use log::{error, info};
use polodb_core::bson::doc;
use polodb_core::{Collection, Database};
// use polodb_core::Database;

pub mod atstypes;
pub mod server_functions;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/insert_est")]
async fn insert_est(info: web::Json<atstypes::EstInInfo>) -> impl Responder {
    let estid = server_functions::gen_id(info.comment.clone());
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("Could not open db file");
    let acctid = server_functions::check_for_existing_account(&db, info.email.clone());
    let estscoll = &db.collection("estimates");
    let est = atstypes::EstOutInfo {
        acctid: acctid.await,
        estid: estid.clone(),
        name: info.name.clone(),
        addr: info.addr.clone(),
        city: info.city.clone(),
        phone: info.phone.clone(),
        email: info.email.clone(),
        reqservdate: info.reqservdate.clone(),
        comment: info.comment.clone(),
        completed: String::from("No"),
    };
    info!(target: "atsrevserver", "insert_est est: {:?}", est);
    estscoll.insert_one(est).expect("unable to insert revs");

    HttpResponse::Ok().body("Insert estimate is complete")
}

#[get("/allests")]
async fn allests() -> impl Responder {
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("Could not open db file");
    let estscoll = db.collection::<atstypes::EstOutInfo>("estimates");
    let ests = estscoll
        .find(doc! {"completed": "No"})
        .expect("could not find ests");
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

#[get("/completed")]
async fn completed(data: web::Query<atstypes::EstData>) -> impl Responder {
    let estid = data.estid.clone();
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("Could not open db file");
    let estscoll: Collection<atstypes::EstOutInfo> = db.collection("estimates");
    let now = Local::now();
    let completion_date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    estscoll
        .update_many(
            doc! {"estid": estid},
            doc! {"$set": {"completed": completion_date}},
        )
        .expect("unable to update ests");

    HttpResponse::Ok().body("Completed")
}

#[post("/insert_rev")]
async fn insert_review(info: web::Json<atstypes::RevInInfo>) -> impl Responder {
    let rev_str = serde_json::to_string(&info.review.clone()).expect("unable to serialize revs");
    let revid = server_functions::gen_id(rev_str);
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("Could not open db file");
    let acctid = server_functions::check_for_existing_account(&db, info.email.clone());
    let revscoll = &db.collection("reviews");
    info!(target: "atsrevserver", "insert_review boo: {:?}", revid);
    revscoll
        .insert_one(atstypes::RevOutInfo {
            acctid: acctid.await,
            revid: revid.clone(),
            name: info.name.clone(),
            email: info.email.clone(),
            stars: info.stars.clone(),
            review: info.review.clone(),
        })
        .expect("unable to insert revs");

    HttpResponse::Ok().body("ReviewInserted")
}

#[get("allrevs")]
async fn allrevs() -> impl Responder {
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("could not open db file");
    let allrevscoll = db.collection::<atstypes::RevOutInfo>("reviews");
    let revs = allrevscoll.find(None).expect("could not find reives");
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

    HttpResponse::Ok().json(arevs)
}

#[get("/accept")]
async fn accept() -> impl Responder {
    HttpResponse::Ok().body("Accept")
}

#[get("/reject")]
async fn reject() -> impl Responder {
    HttpResponse::Ok().body("Reject")
}
