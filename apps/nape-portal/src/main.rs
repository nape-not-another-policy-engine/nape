use leptos::mount::mount_to_body;
use crate::app::App;

mod app;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    mount_to_body(App);
}
