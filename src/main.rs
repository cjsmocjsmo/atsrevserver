use actix_files as fs;
use actix_web::{web, App, HttpServer};
use std::env;

pub mod func;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let img_path = env::var("FIRE_THUMBNAILS").unwrap();

    HttpServer::new(move || {
        App::new()
            .service(crate::func::server_functions::hello)
            .service(crate::func::server_functions::allrevs)
            .service(crate::func::server_functions::allests)
            
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