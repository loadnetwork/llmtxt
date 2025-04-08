use std::path::Path;

pub fn sanitize_path(path: &str) -> String {
    path.replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], "_")
}

pub fn is_text_file(path: &str) -> bool {
    let text_extensions = [
        ".txt",
        ".md",
        ".rs",
        ".js",
        ".py",
        ".html",
        ".css",
        ".json",
        ".toml",
        ".yaml",
        ".yml",
        ".sh",
        ".bat",
        ".c",
        ".cpp",
        ".h",
        ".hpp",
        ".java",
        ".go",
        ".ts",
        ".tsx",
        ".jsx",
        ".php",
        ".rb",
        ".pl",
        ".sql",
        ".gitignore",
        ".env",
        ".ini",
        ".conf",
        ".cfg",
        ".service",
        ".lock",
    ];

    let path = Path::new(path);
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            let dot_ext = format!(".{}", ext_str.to_lowercase());
            return text_extensions.contains(&dot_ext.as_str());
        }
    }

    let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    let common_text_files = [
        "readme",
        "license",
        "dockerfile",
        "makefile",
        "gemfile",
        "rakefile",
    ];

    common_text_files
        .iter()
        .any(|&name| filename.to_lowercase() == name)
}

pub fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size < KB {
        format!("{} B", size)
    } else if size < MB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else {
        format!("{:.2} GB", size as f64 / GB as f64)
    }
}
