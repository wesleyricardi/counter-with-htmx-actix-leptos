use crate::components::counter::CounterComponent;
use leptos::*;

pub async fn page_index(count: u8) -> String {
    leptos::ssr::render_to_string(move |cx| {
        view! { cx,
          <head>
              <meta charset="utf-8"/>
              <meta http-equiv="X-UA-Compatible" content="IE=edge"/>
              <title>"Counter using htmx + actix + leptos"</title>
              <meta name="viewport" content="width=device-width, initial-scale=1"/>
              <link rel="stylesheet" type="text/css" media="screen" href="static/styles/output.css"/>
              <script src="https://unpkg.com/htmx.org@1.9.4" integrity="sha384-zUfuhFKKZCbHTY6aRR46gxiqszMk5tcHjsVFxnUo8VMus4kHGVdIYVbOYYNlKmHV"
              crossorigin="anonymous"></script>
          </head>
          <body>
              <div class="bg-slate-800 h-screen flex justify-around items-center flex-col">
                  <h1 class="text-4xl text-slate-200">
                  "Counter"
                  </h1>
                  <CounterComponent count=count />
              </div>
          </body>
        }
    })
}
