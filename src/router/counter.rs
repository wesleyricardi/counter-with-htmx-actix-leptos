use actix_web::{get, post, web, HttpResponse, HttpRequest};

use crate::{pages::index::page_index, AppState};

#[get("/")]
pub async fn counter(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let count = data.counter.lock().unwrap();
    let count_clone = *count;

    let html = page_index(count_clone).await;

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

#[post("/increase")]
pub async fn increase(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let mut count = data.counter.lock().unwrap(); 
    *count += 1; 

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(format!("{}", count))
}