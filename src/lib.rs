pub mod app;
pub mod app_state;
pub mod server_functions;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}
