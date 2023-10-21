mod app;
use app::*;
use leptos::*;

fn main() {
    // Use leptos as frontend html builder
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
