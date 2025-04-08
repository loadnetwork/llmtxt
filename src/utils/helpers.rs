use std::path::Path;

pub fn sanitize_path(path: &str) -> String {
    path.replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], "_")
}

pub fn is_text_file(path: &str) -> bool {
    let text_extensions = [
        ".rs", ".toml", ".c", ".cpp", ".cc", ".cxx", ".h", ".hpp", ".hxx", ".hh",
        ".js", ".jsx", ".ts", ".tsx", ".mjs", ".cjs", ".html", ".htm", ".css", 
        ".scss", ".sass", ".less", ".svg", ".xml", ".py", ".pyi", ".pyx", ".pxd",
        ".go", ".mod", ".java", ".kt", ".kts", ".scala", ".sc", ".rb", ".rake", 
        ".gemspec", ".php", ".phtml", ".php5", ".php7", ".cs", ".csx", ".fs", 
        ".fsx", ".vb", ".sh", ".bash", ".zsh", ".fish", ".bat", ".cmd", ".ps1",
        ".erl", ".hrl", ".ex", ".exs", ".lisp", ".cl", ".clj", ".cljs", ".cljc", 
        ".edn", ".hs", ".lhs", ".swift", ".dart", ".lua", ".r", ".rmd", ".pl", 
        ".pm", ".sql", ".json", ".yaml", ".yml", ".xml", ".ini", ".conf", ".cfg", 
        ".properties", ".md", ".markdown", ".txt", ".csv", ".tsv", ".gitignore", 
        ".gitattributes", ".gitmodules", ".editorconfig", ".lock",
    ];
    
    let binary_extensions = [
        ".exe", ".dll", ".so", ".dylib", ".obj", ".o", ".a", ".lib",
        ".bin", ".dat", ".db", ".sqlite", ".dbf", ".mdb", ".accdb",
        ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".tiff", ".ico", ".webp",
        ".mp3", ".mp4", ".wav", ".ogg", ".flac", ".avi", ".mov", ".mkv",
        ".zip", ".tar", ".gz", ".bz2", ".xz", ".7z", ".rar",
        ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx",
        ".class", ".pyc", ".pyo", ".pyd", ".wasm",
    ];

    let path = Path::new(path);
    
    // Check if it's a known text or binary file by extension
    if let Some(extension) = path.extension() {
        if let Some(ext_str) = extension.to_str() {
            let dot_ext = format!(".{}", ext_str.to_lowercase());
            if text_extensions.contains(&dot_ext.as_str()) {
                return true;
            }
            if binary_extensions.contains(&dot_ext.as_str()) {
                return false;
            }
        }
    }
    
    // Get the filename for special case handling
    let filename = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
    let filename_lower = filename.to_lowercase();
    
    // Special case handling for git hooks and other common script files without extensions
    if filename_lower.starts_with("pre-") || 
       filename_lower.starts_with("post-") || 
       filename_lower.contains("commit") || 
       filename_lower.contains("hook") {
        return true;
    }
    
    let common_text_files = [
        "readme", "license", "dockerfile", "makefile", "gemfile", "rakefile",
        "vagrantfile", "procfile", "jenkinsfile", "brewfile", "justfile",
        "cargo.lock", "package-lock.json", "yarn.lock", "gemfile.lock",
        ".gitignore", ".gitattributes", ".gitmodules", ".editorconfig",
        ".eslintrc", ".prettierrc", ".babelrc", ".stylelintrc", ".npmrc",
        "cmakelists.txt", "configure", "install", "uninstall",
    ];
    
    if common_text_files.iter().any(|&name| filename_lower == name) {
        return true;
    }
    
    // Consider files without extensions as text files by default
    if !path.extension().is_some() {
        return true;
    }
    
    true
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
