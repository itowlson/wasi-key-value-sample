#[allow(warnings)]
mod bindings;

use bindings::exports::example::qr::qr::Guest;
use bindings::wasi::keyvalue::store::Bucket;

struct Component;

impl Guest for Component {
    fn get_qr_code(bucket: Bucket, url: String, size: u32) -> String {
        // Delimit the size using characters that can't be part of a URL
        let cache_key = format!("{url}<{size}>");

        if let Ok(Some(cached)) = bucket.get(&cache_key) {
            return String::from_utf8_lossy(&cached).to_string();
        }

        // It's not in the cache. Generate it and cache it.
        let svg = qr::generate_qr_code(&url, size);
    
        let _ = bucket.set(&cache_key, svg.as_bytes()); // Failure to cache is not an error

        return svg;
    }
}

bindings::export!(Component with_types_in bindings);

mod qr {
    pub fn generate_qr_code(url: &str, size: u32) -> String {
        format!("<a>SVG <string /> for URL {url} of <size value={size} /></a>")
    }
}
