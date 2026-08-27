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

    fn generatetofile(markdown: String, filepath: String) -> Result<(), String> {
        markdown2pdf::parse_into_file(markdown, filepath, ConfigSource::Default, None)
            .map_err(|e| e.to_string())
    }
}

export!(Conversion);

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_generate() {
        let md = "# Hello World\n\nThis is a test markdown.".to_string();
        let result = Conversion::generate(md);
        assert!(result.is_ok());
        let pdf_bytes = result.unwrap();
        assert!(!pdf_bytes.is_empty());
        assert_eq!(&pdf_bytes[0..4], b"%PDF");
    }

    #[test]
    fn test_generate_to_file() {
        let filepath =
            std::env::temp_dir().join(format!("wasi_component_md_pdf_{}.pdf", std::process::id()));

        Conversion::generatetofile("# Hello World".to_string(), filepath.display().to_string())
            .unwrap();

        let pdf_bytes = std::fs::read(&filepath).unwrap();
        std::fs::remove_file(filepath).unwrap();
        assert_eq!(&pdf_bytes[0..4], b"%PDF");
    }
}
