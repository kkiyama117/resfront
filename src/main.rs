mod app;

use app::*;
use leptos::*;

// Use leptos as frontend html builder
fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
