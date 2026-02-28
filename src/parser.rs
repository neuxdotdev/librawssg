use anyhow::Result;
use std::fs;
use std::path::Path;

/// Struktur konten .rw
#[derive(Debug)]
pub struct RawContent {
    pub frontmatter: Option<String>,
    pub body: String,
}

/// Parsing file .rw sederhana: jika ada baris "---" pisahkan frontmatter dan body
pub fn parse_rw_file(path: &Path) -> Result<RawContent> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Cek apakah ada frontmatter (dimulai dengan "---")
    if lines.first() == Some(&"---") {
        // Cari penutup "---" berikutnya
        let mut end_index = None;
        for (i, line) in lines.iter().enumerate().skip(1) {
            if *line == "---" {
                end_index = Some(i);
                break;
            }
        }
        if let Some(idx) = end_index {
            let frontmatter = lines[1..idx].join("\n");
            let body = lines[idx+1..].join("\n");
            Ok(RawContent {
                frontmatter: Some(frontmatter),
                body,
            })
        } else {
            // Tidak ada penutup, anggap semua body
            Ok(RawContent {
                frontmatter: None,
                body: content,
            })
        }
    } else {
        Ok(RawContent {
            frontmatter: None,
            body: content,
        })
    }
}