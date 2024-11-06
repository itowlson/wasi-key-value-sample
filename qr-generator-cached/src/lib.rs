#[allow(warnings)]
mod bindings;

use bindings::exports::example::qr::qr::Guest;
use bindings::wasi::keyvalue::store;

struct Component;

impl Guest for Component {
    fn get_qr_code(url: String, size: u32) -> String {
        let store_name = "default";

        // Delimit the size using characters that can't be part of a URL
        let cache_key = format!("{url}<{size}>");

        if let Ok(store) = store::open(&store_name) {
            if let Ok(Some(cached)) = store.get(&cache_key) {
                return String::from_utf8_lossy(&cached).to_string();
            }

            // It's not in the cache. Generate it and cache it.
            let svg = qr::generate_qr_code(&url, size);
        
            let _ = store.set(&cache_key, svg.as_bytes()); // Failure to cache is not an error

            return svg;
        }

        qr::generate_qr_code(&url, size)
    }
}

bindings::export!(Component with_types_in bindings);

mod qr {
    pub fn generate_qr_code(url: &str, size: u32) -> String {
        format!("<a>SVG <string /> for URL {url} of <size value={size} /></a>")
    }
}
