use std::path::PathBuf;
use super::*;
use tera::Context;
use walkdir::{WalkDir, DirEntry};

pub fn generate() {
    let content_dir = PathBuf::from(&CONTENT_PATH.as_str());
    let content_dir_walker = WalkDir::new(content_dir).into_iter();
    let mut context = Context::new();

    let site = json!({
        "name": *SITE_NAME,
    });

    context.add("site", &site);

    for entry in content_dir_walker {
        let entry = entry.unwrap();
        if is_hidden(&entry) || !is_valid_format(&entry) { continue; }

        let content = io::read(&entry.path());
        if let Ok(page) = parser::page(content) {
            let mut page_context = context.clone();
            page_context.add("page", &page.frontmatter);
            page_context.add("content", &page.content);
            render(page_context);
        };
    }
}

fn render(context: Context) {
    match TEMPLATES.render("views/post.html", context) {
        Ok(s) => println!("{:?}", s),
        Err(e) => {
            println!("Error: {}", e);
            for e in e.iter().skip(1) {
                println!("Reason: {}", e);
            }
        }
    };
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn is_valid_format(entry: &DirEntry) -> bool {
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
