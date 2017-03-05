#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate dotenv;
extern crate globset;
extern crate pulldown_cmark as cmark;
extern crate regex;
extern crate serde;
extern crate tera;
extern crate walkdir;

use dotenv::dotenv;
use globset::GlobMatcher;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use tera::Tera;

#[derive(Debug, Serialize)]
pub struct Page {
    pub frontmatter: Value,
    pub content: String,
    pub original_path_string: String,
}

#[derive(Debug)]
pub struct View {
    pub target: GlobMatcher,
    pub template: String,
    pub custom_loops: HashMap<String, String>,
}

lazy_static! {
    pub static ref BUILD_PATH: String = env::var("BUILD_PATH").unwrap();
    pub static ref CONTENT_PATH: String = env::var("CONTENT_PATH").unwrap();
    pub static ref SITE_NAME: String = env::var("SITE_NAME").unwrap();
    pub static ref TEMPLATE_DATA: HashMap<String, View> = views::get_data();
    pub static ref TEMPLATES: Tera = views::get_tera();
    pub static ref THEME_PATH: String = env::var("THEME_PATH").unwrap();
    pub static ref GLOB_FOR_LOOP_REGEX: Regex = Regex::new(r#"(\{%\sfor\s\w*\sin)\s(")([^\s]*)(")\s(%})"#).unwrap();
}

mod generator;
mod io;
mod parser;
mod utils;
mod views;

fn main() {
    dotenv().ok();
    generator::generate();
}
