use cmark::html::push_html;
use cmark::Parser;
use globset::{Glob, GlobMatcher};
use regex::Captures;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use super::*;

pub fn page(page_string: String, is_markdown: bool) -> Result<(Value, String), Error> {
    if let Ok((frontmatter, mut content)) = separate_frontmatter(page_string) {
        if is_markdown {
            content = parse_markdown(&content);
        }

        return Ok((serde_json::to_value(&frontmatter).unwrap(), content));
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to parse a page"))
}

pub fn view(view_string: String) -> Result<View, Error> {
    if let Ok((mut target, mut template)) = separate_target(view_string) {
        let (template, custom_loop_values) = extract_custom_loops(&mut template);

        let view = View {
            target: parse_target(&mut target).unwrap(),
            template: template,
            custom_loops: custom_loop_values,
        };

        return Ok(view);
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to parse a view"))
}

fn extract_custom_loops(template: &mut String) -> (String, HashMap<String, String>) {
    let mut custom_loops = HashMap::new();
    let mut custom_loop_count: u32 = 0;

    *template = GLOB_FOR_LOOP_REGEX.replace_all(template, |captures: &Captures| {
        let glob = captures.get(2).map_or("", |m| m.as_str()).to_string();
        let id = format!("__CUSTOM_LOOP_{}__", custom_loop_count);

        custom_loops.insert(glob, id.clone());
        custom_loop_count += 1;

        return format!("{} {} {}", captures.get(1).map_or("", |m| m.as_str()), id, captures.get(3).map_or("", |m| m.as_str()));
    }).into_owned();

    (template.to_string(), custom_loops)
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
