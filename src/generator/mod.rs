use std::collections::HashMap;
use std::path::{Path, PathBuf};
use super::*;
use tera::Context;
use walkdir::WalkDir;

pub fn generate() {
    let content = map_sites_content();
    let context = get_sites_context();

    render_from_views(content, context);
}

fn map_sites_content() -> HashMap<String, Page> {
    let mut content = HashMap::new();
    let content_dir = PathBuf::from(&CONTENT_PATH.as_str());
    let content_dir_walker = WalkDir::new(content_dir).into_iter().filter_map(|e| e.ok());

    for entry in content_dir_walker {
        if utils::is_hidden_file(&entry) || !utils::is_valid_file_format(&entry) { continue; }

        let file_path_str = &entry.path().to_str().unwrap()[CONTENT_PATH.len()+1..]; // +1 removes the leftover `/`
        let file_contents = io::read(entry.path());

        if let Ok((frontmatter, page_content)) = parser::page(file_contents, utils::is_markdown_file(&entry)) {
            let page = Page {
                frontmatter: frontmatter,
                content: page_content,
                original_path_string: entry.path().to_str().unwrap().to_string(),
            };

            content.insert(file_path_str.to_string(), page);
        };
    }

    content
}

fn get_sites_context() -> Context {
    let mut context = Context::new();

    let site = json!({
        "name": *SITE_NAME,
    });

    context.add("site", &site);

    context
}

fn render_from_views(content: HashMap<String, Page>, context: Context) {
    for view in TEMPLATE_PATH_DATA.iter() {
        for page in &content {
            if view.1.is_match(page.0) {
                let mut page_context = context.clone();
                page_context.add("page", &page.1.frontmatter);
                page_context.add("content", &page.1.content);

                if let Some(rendered) = render(page_context, view.0) {
                    io::write(Path::new(&page.1.original_path_string), rendered);
                }
            }
        }

    }
}

fn render(context: Context, template: &str) -> Option<String> {
    match TEMPLATES.render(template, context) {
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
