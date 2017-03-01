use std::path::PathBuf;
use super::*;
use tera::Context;
use walkdir::WalkDir;

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
        if utils::is_dotfile(&entry) || !utils::is_valid_file_format(&entry) { continue; }

        let content = io::read(entry.path());
        if let Ok(page) = parser::page(content, utils::is_markdown_file(&entry)) {
            let mut page_context = context.clone();
            page_context.add("page", &page.frontmatter);
            page_context.add("content", &page.content);

            if let Some(rendered) = render(page_context) {
                io::write(entry.path(), rendered);
            }
        };
    }
}

fn render(context: Context) -> Option<String> {
    match TEMPLATES.render("views/post.html", context) {
        Ok(html) => Some(html),
        Err(e) => {
            println!("Error: {}", e);

            for e in e.iter().skip(1) {
                println!("Reason: {}", e);
            }

            None
        }
    }
}
