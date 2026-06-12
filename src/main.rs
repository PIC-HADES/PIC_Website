use leptos::prelude::*;

mod app;
mod pages;
mod components;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    view! { <app::MissaoHadesApp/> }
}
