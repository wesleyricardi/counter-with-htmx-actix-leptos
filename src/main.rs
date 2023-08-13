use std::sync::Mutex;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, HttpRequest};
use leptos::*;

struct AppState {
    counter: Mutex<u8>,
}


#[component]
pub fn CounterComponent(cx: Scope, count: u8) -> impl IntoView {
    view! {
        cx,
        <main>
        <div id="counting">
        {count}</div>
        <button hx-post="/increase"
            hx-trigger="click"
            hx-target="#counting"
            hx-swap="innerHTML"
        >
            "Click Me!"
        </button>
        </main>
    }
}

#[get("/")]
async fn counter(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let count = data.counter.lock().unwrap();
    let count_clone = *count;

    let html = leptos::ssr::render_to_string(move |cx| view! { cx,
        <head>
            <meta charset="utf-8"/>
            <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
            <title>"Counter using htmx + actix + leptos"</title>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <link rel="stylesheet" type="text/css" media="screen" href="main.css"/>
            <script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV" 
            crossorigin="anonymous"></script>
        </head>
        <body>
            <CounterComponent count=count_clone />
        </body>
      });

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(html)
}

#[post("/increase")]
async fn increase(_req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let mut count = data.counter.lock().unwrap(); 
    *count += 1; 

    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(format!("{}", count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
    });


    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(counter)
            .service(increase)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}