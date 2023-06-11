use chrono::Local;
use log::error;
use md5;
use polodb_core::Database;
use polodb_core::bson::doc;
// use serde::{Deserialize, Serialize};
use crate::func::atstypes;

pub fn gen_id(astring: String) -> String {
    let result = md5::compute(astring);
    let hstring = format!("{:x}", result);

    hstring
}

pub async fn create_account(qemail: String) -> String {
    let acct = qemail.clone();
    let acctid = gen_id(qemail.clone());
    let now = Local::now();
    let creation_date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("unable to open db file for acccounts");
    let acctcoll = db.collection("accounts");
    acctcoll
        .insert_one(atstypes::Account {
            acct: acct,
            acctid: acctid.clone(),
            creation_date: creation_date,
        })
        .expect("Account insertion has failed");

    acctid.clone()
}


pub async fn check_for_existing_account(qemail: String) -> String {
    let db = Database::open_file("/home/pipi/atsrevserver/ats.db").expect("Could not open db file");
    let acctcoll = db.collection::<atstypes::Account>("accounts");
    let accts = acctcoll.find(doc! {"acct": qemail.clone()}).expect("Unable to find account");
    
    
    let mut acct_vec = Vec::new();
    for ac in accts {
        let ac_result = ac;
        let res = match ac_result {
            Ok(ac) => ac,
            Err(e) => {
                error!(target: "atsrevserver", "check_for_existing_account error: {:?}", e);
                continue;
            }
        };
        acct_vec.push(res);
    }

    if acct_vec.len() > 0 {
        let accidlist = serde_json::to_string(&acct_vec).expect("unable to serialize revs");
        return accidlist
    } else {
        let accidlist = create_account(qemail.clone()).await;
        // let accidlist = String::from("None");
        return accidlist
    }
    // let accidlist = erde_json::to_string(&act_vec).expect("unable to serialize revs");

    // accidlist
}

