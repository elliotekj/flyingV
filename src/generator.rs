use globset::Glob;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use super::*;
use tera::Context;
use walkdir::WalkDir;

pub fn generate() {
    let mapped_site_content = map_sites_content();
    let tera_context = get_sites_context();

    render_from_views(mapped_site_content, tera_context);
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
    let mut tera_context = Context::new();

    let site = json!({
        "name": *SITE_NAME,
    });

    tera_context.add("site", &site);

    tera_context
}

fn render_from_views(mapped_site_content: HashMap<String, Page>, tera_context: Context) {
    for view in VIEW_DATA.iter() {
        let view_data = view.1;

        for page in &mapped_site_content {
            if view_data.target.is_match(page.0) {
                let mut page_context = tera_context.clone();
                page_context.add("page", &page.1.frontmatter);
                page_context.add("content", &page.1.content);

                for custom_loop in &view_data.custom_loops {
                    let glob = Glob::new(custom_loop.0).unwrap().compile_matcher();
                    let mut loop_data = Vec::new();

                    for page_data in &mapped_site_content {
                        if glob.is_match(page_data.0) {
                            loop_data.push(page_data.1);
                        }
                    }

                    page_context.add(custom_loop.1, &loop_data.to_owned());
                }

                if let Some(rendered) = render(page_context, view.0) {
                    io::write(Path::new(&page.1.original_path_string), rendered);
                }
            }
        }
    }

    views::destroy_tmp_views_dir();
}

fn render(page_context: Context, view_path_str: &str) -> Option<String> {
    match TERA.render(view_path_str, page_context) {
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