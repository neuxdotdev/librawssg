use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::parser;

/// Build site dari source ke output dengan template dari template_dir
pub fn build_site(
    source: &str,
    output: &str,
    template_dir: &str,
    _configg: Option<String>,
    verbose: bool,
) -> Result<()> {
    let source_path = Path::new(source);
    let output_path = Path::new(output);
    let template_path = Path::new(template_dir);

    if !source_path.exists() {
        anyhow::bail!("Source directory '{}' does not exist", source);
    }

    // Baca template dasar
    let base_template = fs::read_to_string(template_path.join("base.html"))
        .unwrap_or_else(|_| DEFAULT_BASE_HTML.to_string());
    let index_template = fs::read_to_string(template_path.join("index.html"))
        .unwrap_or_else(|_| DEFAULT_INDEX_HTML.to_string());

    // Proses setiap file .rw di source
    for entry in WalkDir::new(source_path) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("rw") {
            if verbose {
                println!("Processing: {}", path.display());
            }

            // Parse konten
            let raw = parser::parse_rw_file(path)?;

            // Tentukan path output
            let rel_path = path.strip_prefix(source_path)?;
            let mut out_file = output_path.join(rel_path);
            out_file.set_extension("html");

            // Buat direktori jika perlu
            if let Some(parent) = out_file.parent() {
                fs::create_dir_all(parent)?;
            }

            // Render HTML
            let html = render_page(&raw.body, &base_template, &index_template)?;
            fs::write(out_file, html)?;
        }
    }

    Ok(())
}

fn render_page(body: &str, base_template: &str, index_template: &str) -> Result<String> {
    // Sederhana: ganti placeholder {{ content }} dengan body
    let content_html = format!("<pre>{}</pre>", body);
    let page = index_template.replace("{{ content }}", &content_html);
    let final_html = base_template.replace("{{ content }}", &page);
    Ok(final_html)
}

// Template default jika tidak ditemukan
const DEFAULT_BASE_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RAWSSG Site</title>
    <link rel="stylesheet" href="/style/main.css">
</head>
<body>
    <main>
        {{ content }}
    </main>
    <script src="/scripts/scripts.js"></script>
</body>
</html>"#;

const DEFAULT_INDEX_HTML: &str = r#"<article>
    {{ content }}
</article>"#;