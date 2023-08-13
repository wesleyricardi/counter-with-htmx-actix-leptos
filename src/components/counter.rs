use leptos::*;

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
