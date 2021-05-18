extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use js_sys::Promise;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub async fn fromRS() -> Result<String, String> {
    utils::set_panic_hook();
    Ok(JsFuture::from(EXAMPLE_NS::get("foo"))
        .await
        .unwrap()
        .as_string()
        .unwrap())
}

#[wasm_bindgen]
extern "C" {
    type EXAMPLE_NS;

    #[wasm_bindgen(static_method_of = EXAMPLE_NS)]
    fn get(key: &str) -> Promise;
}
