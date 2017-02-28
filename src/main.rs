#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate tera;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use tera::Tera;

lazy_static! {
    pub static ref SITE_NAME: String = env::var("SITE_NAME").unwrap();
    pub static ref THEME_PATH: String = env::var("THEME_PATH").unwrap();
    pub static ref TEMPLATES: Tera = compile_templates!(format!("{}/**/*.html", *THEME_PATH).as_str());
}

mod generator;

fn main() {
    dotenv().ok();
    generator::generate();
}
