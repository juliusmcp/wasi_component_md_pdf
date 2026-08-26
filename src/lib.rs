wit_bindgen::generate!({
    path: "conversion.wit",
    world: "conversion",
});

use markdown2pdf::config::ConfigSource;

#[allow(dead_code)]
struct Conversion;

impl Guest for Conversion {
    fn generate(markdown: String) -> Result<Vec<u8>, String> {
        markdown2pdf::parse_into_bytes(markdown, ConfigSource::Default, None)
            .map_err(|e| e.to_string())
    }
}

export!(Conversion);