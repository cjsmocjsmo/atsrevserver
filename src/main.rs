// use actix_files as fs;
use actix_web::{App, HttpServer};
use actix_cors::Cors;
// use std::env;
// use log::{error, info, debug};

pub mod func;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
   
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(crate::func::hello)
            .service(crate::func::insert_review)
            .service(crate::func::allrevs)
            .service(crate::func::allests)
            
            // .route(
            //     "/hey",
            //     web::get().to(crate::func::server_functions::manual_hello),
            // )
            
            // .service(fs::Files::new("/thumbnails", img_path.clone()).show_files_listing())
        }
    )
    .bind(("192.168.0.91", 8080))?
    .run()
    .await
}