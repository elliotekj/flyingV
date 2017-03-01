use walkdir::DirEntry;

pub fn is_dotfile(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn is_valid_file_format(entry: &DirEntry) -> bool {
    let path = entry.path();

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

pub fn is_markdown_file(entry: &DirEntry) -> bool {
    let extension = entry.path().extension().unwrap();

    if extension == "markdown" || extension == "md" {
        return true;
    }

    false
}

pub fn is_html_file(entry: &DirEntry) -> bool {
    let path = entry.path();

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
