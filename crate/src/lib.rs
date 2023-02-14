pub mod read;
extern crate cfg_if;
extern crate wasm_bindgen;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    log("wasm start".into());
}

///file: 前端File对象
#[wasm_bindgen]
pub async fn get_json(f: web_sys::File) -> Result<String, JsValue> {
    let blob = f.slice()?;
    let buf =
        js_sys::Uint8Array::new(&wasm_bindgen_futures::JsFuture::from(blob.array_buffer()).await?)
            .to_vec();
    Ok(read::wasm_convert(&buf)?)
}
