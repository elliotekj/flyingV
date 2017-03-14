use chrono::DateTime;
use cmark::html::push_html;
use cmark::Parser;
use globset::{Glob, GlobMatcher};
use regex::Captures;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use super::*;

pub fn page(page_string: String, is_markdown: bool) -> Result<(Value, Option<i64>, String), Error> {
    if let Ok((parsed_frontmatter, mut content)) = separate_frontmatter(page_string) {
        if is_markdown {
            content = parse_markdown(&content);
        }

        return Ok((serde_json::to_value(&parsed_frontmatter.frontmatter).unwrap(), parsed_frontmatter.timestamp, content));
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to parse a page"))
}

pub fn view(view_string: String) -> Result<View, Error> {
    if let Ok((mut target, mut html)) = separate_target(view_string) {
        let (html, custom_loop_values) = extract_custom_loops(&mut html);

        let view = View {
            target: parse_target(&mut target).unwrap(),
            html: html,
            custom_loops: custom_loop_values,
        };

        return Ok(view);
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to parse a view"))
}

fn extract_custom_loops(html: &mut String) -> (String, HashMap<String, String>) {
    let mut custom_loops = HashMap::new();
    let mut custom_loop_count: u32 = 0;

    *html = GLOB_FOR_LOOP_REGEX.replace_all(html, |captures: &Captures| {
        let glob = captures.get(2).map_or("", |m| m.as_str()).to_string();
        let id = format!("__CUSTOM_LOOP_{}__", custom_loop_count);

        custom_loops.insert(glob, id.clone());
        custom_loop_count += 1;

        return format!("{} {} {}", captures.get(1).map_or("", |m| m.as_str()), id, captures.get(3).map_or("", |m| m.as_str()));
    }).into_owned();

    (html.to_string(), custom_loops)
}

fn separate_frontmatter(page_string: String) -> Result<(ParsedFrontmatter, String), Error> {
    if let Some(frontmatter_len) = page_string.find("\n\n") {
        let frontmatter_string = &page_string[..frontmatter_len];
        let content = &page_string[frontmatter_len..];
        let parsed_frontmatter = parse_frontmatter(frontmatter_string);

        if !parsed_frontmatter.frontmatter.is_empty() {
            return Ok((parsed_frontmatter, content.to_owned()));
        }
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to build due to missing frontmatter"))
}

fn separate_target(view_string: String) -> Result<(String, String), Error> {
    if let Some(target_len) = view_string.find('\n') {
        let target_string = &view_string[..target_len];
        let html_string = &view_string[target_len+1..]; // +1 to clear out the remaining \n

        return Ok((target_string.to_owned(), html_string.to_owned()));
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to build due to missing frontmatter"))
}

fn parse_frontmatter(frontmatter_string: &str) -> ParsedFrontmatter {
    let mut frontmatter = HashMap::new();
    let mut timestamp: Option<i64> = None;
    let frontmatter_lines = frontmatter_string.lines();

    for line in frontmatter_lines {
        let split: Vec<&str> = line.split(':').collect();
        let key = split[0];
        let value = split[1..].join(":");

        if key == "datetime" {
            let datetime = format!("{} {}", value, *UTC_OFFSET);

            match DateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M:%S %z") {
                Ok(datetime) => {
                    timestamp = Some(datetime.timestamp());
                },
                Err(_) => {
                    // TODO: Add a proper error here.
                },
            }
        }

        frontmatter.insert(key.trim().to_string(), value.trim().to_string());
    }

    ParsedFrontmatter {
        frontmatter: frontmatter,
        timestamp: timestamp,
    }
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
