#[macro_use] extern crate clap;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
extern crate chrono;
extern crate dotenv;
extern crate globset;
extern crate hyper;
extern crate notify;
extern crate pulldown_cmark as cmark;
extern crate regex;
extern crate serde;
extern crate tempdir;
extern crate tera;
extern crate walkdir;

use clap::App;
use dotenv::dotenv;
use globset::GlobMatcher;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::thread;
use tera::Tera;

#[derive(Debug, Serialize)]
pub struct Page {
    pub fm: Value, // frontmatter
    pub content: String,
    pub url: String,
    pub timestamp: Option<i64>,
}

#[derive(Debug)]
pub struct View {
    pub target: GlobMatcher,
    pub html: String,
    pub custom_loops: HashMap<String, String>,
}

pub struct ParsedFrontmatter {
    pub fm: HashMap<String, Value>, // frontmatter
    pub timestamp: Option<i64>,
}

lazy_static! {
    pub static ref BUILD_PATH: String = env::var("BUILD_PATH").unwrap();
    pub static ref CONTENT_PATH: String = env::var("CONTENT_PATH").unwrap();
    pub static ref GLOB_FOR_LOOP_REGEX: Regex = Regex::new(r#"(\{%\sfor\s\w*\sin)\s"([^\s]*)"\s(%})"#).unwrap();
    pub static ref SITE_NAME: String = env::var("SITE_NAME").unwrap();
    pub static ref TERA: Tera = views::get_tera();
    pub static ref THEME_PATH: String = env::var("THEME_PATH").unwrap();
    pub static ref UTC_OFFSET: String = env::var("UTC_OFFSET").unwrap();
    pub static ref VIEW_DATA: HashMap<String, View> = views::get_data();
    pub static ref VIEWS_PATH: String = views::get_path();
    pub static ref VIEWS_TMP_PATH: String = views::get_tmp_path();
}

mod generator;
mod io;
mod parser;
mod server;
mod utils;
mod views;
mod watcher;

fn main() {
    dotenv().ok();

    let clap_config = load_yaml!("cli.yml");
    let clap_matches = App::from_yaml(clap_config).get_matches();

    if clap_matches.subcommand_matches("build").is_some() {
        generator::generate();
    }

    if clap_matches.subcommand_matches("watch").is_some() {
        generator::generate();
        thread::spawn(move || { server::serve(); });

        if let Err(e) = watcher::watch() {
            println!("Failed to start the file watcher: {}", e);
        }
    }
}
