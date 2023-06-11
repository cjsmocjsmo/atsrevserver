use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct EstInInfo {
    pub name: String,
    pub addr: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub reqservdate: String,
    pub comment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstOutInfo {
    pub acctid: String,
    pub estid: String,
    pub name: String,
    pub addr: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub reqservdate: String,
    pub comment: String,
    pub completed: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RevInInfo {
    pub name: String,
    pub email: String,
    pub stars: String,
    pub review: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevOutInfo {
    pub acctid: String,
    pub revid: String,
    pub name: String,
    pub email: String,
    pub stars: String,
    pub review: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub acct: String,
    pub acctid: String,
    pub creation_date: String,
}