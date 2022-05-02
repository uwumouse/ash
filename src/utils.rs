pub fn normalize_zip_path(p: &str) -> String {
    let mut new_str = String::from(p);
    if p.starts_with("/") {
        new_str = p.trim_start_matches("/").to_string();
    }

    new_str
}

/// Returns a touple with size in bigger (if needed) units & unit name for this size
pub fn human_readable_size(mut size: u64) -> (u64, &'static str) {
    let mut unit = "B";
    for i in vec!["KB", "MB", "GB", "TB"] {
        if size > 1024 {
            size /= 1024;
            unit = i;
        }
    }

    (size, unit)
}
