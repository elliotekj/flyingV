use serde_json::{self, Value};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

pub struct Page {
    pub frontmatter: Value,
    pub content: String,
}

pub fn page(page_string: String) -> Result<Page, Error> {
    if let Ok((frontmatter, content)) = separate_frontmatter(page_string) {
        let page = Page {
            frontmatter: serde_json::to_value(&frontmatter).unwrap(),
            content: content,
        };

        return Ok(page);
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "Failed to parse a page"))
    }
}

fn separate_frontmatter(page_string: String) -> Result<(HashMap<String, String>, String), Error> {
    if let Some(frontmatter_len) = page_string.find("\n\n") {
        let frontmatter_string = &page_string[..frontmatter_len];
        let content = &page_string[frontmatter_len..];
        let frontmatter = parse_frontmatter(frontmatter_string);

        if frontmatter.len() > 0 {
            return Ok((frontmatter, content.to_owned()));
        }
    }

    Err(Error::new(ErrorKind::InvalidInput, "Failed to build due to missing frontmatter"))
}

fn parse_frontmatter(frontmatter_string: &str) -> HashMap<String, String> {
    let mut frontmatter = HashMap::new();
    let mut frontmatter_lines = frontmatter_string.lines();

    while let Some(line) = frontmatter_lines.next() {
        let key_value: Vec<&str> = line.split(':').collect();
        frontmatter.insert(key_value[0].trim().to_string(), key_value[1].trim().to_string());
    };

    frontmatter
}
