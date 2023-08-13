use leptos::*;

#[component]
pub fn CounterComponent(cx: Scope, count: u8) -> impl IntoView {
    view! {
        cx,
        <main class="h-80 w-80 rounded-full border-solid border-2 text-purple-600 font-bold border-purple-600 flex flex-col justify-around items-center">
        <h2 class="text-3xl" id="counting">
        {count}</h2>
        <div
         class="text-xl cursor-pointer hover:text-purple-300"
         hx-post="/increase"
            hx-trigger="click"
            hx-target="#counting"
            hx-swap="innerHTML"
        >
            "Click Me!"
        </div>
        </main>
    }
}
