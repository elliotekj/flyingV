use globset::Glob;
use std::cmp::Ordering;
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
        let entry_path = entry.path();
        let entry_path_buf = entry_path.to_path_buf();
        if utils::is_hidden_file(&entry_path_buf) || !utils::is_valid_file_format(&entry_path_buf) { continue; }

        let file_path_str = &entry_path.to_str().unwrap()[CONTENT_PATH.len()+1..]; // +1 removes the leftover `/`
        let file_contents = io::read(entry_path);

        if let Ok((frontmatter, timestamp, page_content)) = parser::page(file_contents, utils::is_markdown_file(&entry_path.to_path_buf())) {
            let page = Page {
                frontmatter: frontmatter,
                content: page_content,
                url: get_url(file_path_str),
                timestamp: timestamp,
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

fn get_url(path_str: &str) -> String {
    let file_path = Path::new(path_str);
    let file_parent = file_path.parent().unwrap().to_str().unwrap();
    let file_stem = file_path.file_stem().unwrap().to_str().unwrap();

    if file_parent == "" && file_stem == "index" {
        String::from("/")
    } else if file_parent == "" {
        format!("/{}", file_stem)
    } else {
        format!("/{}/{}", file_parent, file_stem)
    }
}

fn render_from_views(mapped_site_content: HashMap<String, Page>, tera_context: Context) {
    for (view_path, view_data) in VIEW_DATA.iter() {
        for (path_string, page) in &mapped_site_content {
            if view_data.target.is_match(path_string) {
                let mut page_context = tera_context.clone();
                page_context.add("page", &page.frontmatter);
                page_context.add("content", &page.content);

                for (custom_loop_glob, custom_loop_id) in &view_data.custom_loops {
                    let glob = Glob::new(custom_loop_glob).unwrap().compile_matcher();
                    let mut loop_data = Vec::new();

                    for (path_string, page) in &mapped_site_content {
                        if glob.is_match(path_string) {
                            loop_data.push(page);
                        }
                    }

                    loop_data.sort_by(|a, b| {
                        if a.timestamp == None {
                            return Ordering::Greater;
                        }

                        if b.timestamp == None {
                            return Ordering::Less;
                        }

                        a.timestamp.unwrap().cmp(&b.timestamp.unwrap())
                    });

                    page_context.add(custom_loop_id, &loop_data.to_owned());
                }

                if let Some(rendered) = render(page_context, view_path) {
                    io::write_page(&page.url, rendered);
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
