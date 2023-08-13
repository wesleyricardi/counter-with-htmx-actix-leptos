use std::sync::Mutex;
use actix_files as fs;
use actix_web::{ web, App, HttpServer};
use router::counter::{counter, increase};

mod pages;
mod components;
mod router;

pub struct AppState {
    counter: Mutex<u8>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });


    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .service(counter)
            .service(increase)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}