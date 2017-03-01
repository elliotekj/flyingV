#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate tera;
extern crate dotenv;
extern crate globset;
extern crate pulldown_cmark as cmark;
extern crate walkdir;

use dotenv::dotenv;
use globset::GlobMatcher;
use std::collections::HashMap;
use std::env;
use tera::Tera;

lazy_static! {
    pub static ref BUILD_PATH: String = env::var("BUILD_PATH").unwrap();
    pub static ref CONTENT_PATH: String = env::var("CONTENT_PATH").unwrap();
    pub static ref SITE_NAME: String = env::var("SITE_NAME").unwrap();
    pub static ref TEMPLATES: (HashMap<String, GlobMatcher>, Tera) = views::get();
    pub static ref THEME_PATH: String = env::var("THEME_PATH").unwrap();
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
