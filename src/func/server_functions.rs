use actix_web::{get, post, web, HttpResponse, Responder};
// use actix_web::web::Json;
// use rusqlite::{Connection, Result};
// use serde::Serialize;
// use anyhow::Error;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/allrevs")]
async fn allrevs() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello allrevs</h1>")
}

#[get("/allests")]
async fn allests() -> impl Responder {
    HttpResponse::Ok().body("Hello allests")
}