#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate tera;
extern crate dotenv;
extern crate pulldown_cmark as cmark;
extern crate walkdir;

use dotenv::dotenv;
use std::env;
use tera::Tera;

lazy_static! {
    pub static ref BUILD_PATH: String = env::var("BUILD_PATH").unwrap();
    pub static ref CONTENT_PATH: String = env::var("CONTENT_PATH").unwrap();
    pub static ref THEME_PATH: String = env::var("THEME_PATH").unwrap();
    pub static ref SITE_NAME: String = env::var("SITE_NAME").unwrap();
    pub static ref TEMPLATES: Tera = {
        let mut tera = compile_templates!(format!("{}/**/*.html", *THEME_PATH).as_str());
        tera.autoescape_on(vec![]);
        tera
    };
}

mod generator;
mod io;
mod parser;

fn main() {
    dotenv().ok();
    generator::generate();
}
