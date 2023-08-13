use leptos::*;
use crate::components::counter::CounterComponent;

pub async fn page_index(count:u8) -> String {
    leptos::ssr::render_to_string(move |cx| view! { cx,
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
            <h1 class="text-4xl font-bold underline">
            "Counter"
            </h1>
            <CounterComponent count=count />
        </body>
      })
}