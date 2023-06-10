use chrono::Local;
use md5;
use polodb_core::Database;
use serde::{Deserialize, Serialize};

fn gen_acct_id(astring: String) -> String {
    let result = md5::compute(astring);
    let hstring = format!("{:x}", result);

    hstring
}

#[derive(Serialize, Deserialize)]
struct Account {
    acct: String,
    acctid: String,
    creation_date: String,
}

pub fn create_account(qemail: String) -> String {
    let acct = qemail.clone();
    let acctid = gen_acct_id(qemail.clone());
    let now = Local::now();
    let creation_date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let db = Database::open_file("ats.db").unwrap();
    let acctcoll = db.collection("accounts");
    acctcoll
        .insert_one(Account {
            acct: acct,
            acctid: acctid.clone(),
            creation_date: creation_date,
        })
        .unwrap();

    acctid.clone()
}

