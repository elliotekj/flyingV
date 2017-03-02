use cmark::html::push_html;
use cmark::Parser;
use globset::{Glob, GlobMatcher};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub struct Page {
    pub frontmatter: Value,
    pub content: String,
}

#[derive(Debug)]
pub struct View {
    pub target: GlobMatcher,
    pub template: String,
}

pub fn page(page_string: String, is_markdown: bool) -> Result<Page, Error> {
    if let Ok((frontmatter, mut content)) = separate_frontmatter(page_string) {
        if is_markdown {
            content = parse_markdown(&content);
        }

        let page = Page {
            frontmatter: serde_json::to_value(&frontmatter).unwrap(),
            content: content,
        };

        return Ok(page);
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to parse a page"))
}

pub fn view(view_string: String) -> Result<View, Error> {
    if let Ok((mut target, template)) = separate_target(view_string) {
        let view = View {
            target: parse_target(&mut target).unwrap(),
            template: template,
        };

        return Ok(view);
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to parse a view"))
}

fn separate_frontmatter(page_string: String) -> Result<(HashMap<String, String>, String), Error> {
    if let Some(frontmatter_len) = page_string.find("\n\n") {
        let frontmatter_string = &page_string[..frontmatter_len];
        let content = &page_string[frontmatter_len..];
        let frontmatter = parse_frontmatter(frontmatter_string);

        if !frontmatter.is_empty() {
            return Ok((frontmatter, content.to_owned()));
        }
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to build due to missing frontmatter"))
}

fn separate_target(view_string: String) -> Result<(String, String), Error> {
    if let Some(target_len) = view_string.find('\n') {
        let target_string = &view_string[..target_len];
        let template_string = &view_string[target_len+1..]; // +1 to clear out the remaining \n

        return Ok((target_string.to_owned(), template_string.to_owned()));
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to build due to missing frontmatter"))
}

fn parse_frontmatter(frontmatter_string: &str) -> HashMap<String, String> {
    let mut frontmatter = HashMap::new();
    let frontmatter_lines = frontmatter_string.lines();

    for line in frontmatter_lines {
        let key_value: Vec<&str> = line.split(':').collect();
        frontmatter.insert(key_value[0].trim().to_string(), key_value[1].trim().to_string());
    }

    frontmatter
}

fn parse_markdown(md: &str) -> String {
    let parser = Parser::new(md);
    let mut html = String::new();
    push_html(&mut html, parser);
    html
}

fn parse_target(target: &mut String) -> Result<GlobMatcher, Error> {
    if let Some(markup_start_len) = target.find('"') {
        *target = target[markup_start_len+1..].to_owned();
    }

    if let Some(markup_end_len) = target.find('"') {
        *target = target[..markup_end_len].to_owned();
    }

    if let Ok(target_glob) = Glob::new(target) {
        return Ok(target_glob.compile_matcher());
    };

    Err(Error::new(ErrorKind::InvalidInput, "Failed to parse a glob"))
}
