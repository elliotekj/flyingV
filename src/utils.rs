use std::path::PathBuf;

pub fn is_hidden_file(path: &PathBuf) -> bool {
    path
        .file_name()
        .unwrap()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn is_valid_file_format(path: &PathBuf) -> bool {
    match path.extension() {
        Some(extension) => {
            if extension == "markdown" || extension == "html" || extension == "md" {
                return true
            }

            false
        }
        _ => false,
    }
}

pub fn is_markdown_file(path: &PathBuf) -> bool {
    let extension = path.extension().unwrap();

    if extension == "markdown" || extension == "md" {
        return true;
    }

    false
}

pub fn is_html_file(path: &PathBuf) -> bool {
    match path.extension() {
        Some(extension) => {
            if extension == "html" {
                return true
            }

            false
        }
        _ => false,
    }
}
