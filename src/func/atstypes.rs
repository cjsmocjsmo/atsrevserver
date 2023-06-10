use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct QInfo {
    pub name: String,
    // pub email: String,
    // pub stars: String,
    // pub review: String,
}


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct IInfo {
    // pub acctid: String,
    pub name: String,
    // pub email: String,
    // pub stars: String,
    // pub review: String,
}