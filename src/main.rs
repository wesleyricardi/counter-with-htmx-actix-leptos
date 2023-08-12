use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <main>
           "Hello, World!"
        </main>
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    let html = leptos::ssr::render_to_string(|cx| view! { cx,
        <head>
            <meta charset="utf-8"/>
            <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
            <title>"Hello World!"</title>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <link rel="stylesheet" type="text/css" media="screen" href="main.css"/>
            <script src="main.js"></script>
        </head>
        <body>
            <App/>
        </body>
      });

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}