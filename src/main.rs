use sycamore::prelude::*;

#[derive(Props)]
struct HelloProps {
    name: String,
}

#[component]
fn Greeting(name: HelloProps) -> View {
    view! {
        p { "hello" (name.name) "!"}
    }
}

#[component]
fn App() -> View {
    view! {
        div {
            h1 { "Sycamore 入门" }
            Greeting(name="lyf".into())
        }
    }
}
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(App);
}
