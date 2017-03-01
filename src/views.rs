use std::collections::HashMap;
use std::path::PathBuf;
use super::*;
use tera::Tera;
use walkdir::WalkDir;

pub fn get() -> Tera {
    // let mut view_templates = HashMap::new();
    let views_dir = PathBuf::from(&THEME_PATH.as_str()).join("views");
    let views_dir_walker = WalkDir::new(views_dir).into_iter();

    for entry in views_dir_walker {
        let entry = entry.unwrap();
        if utils::is_dotfile(&entry) || !utils::is_html_file(&entry) { continue; }

        let file = io::read(entry.path());
        let view = parser::view(file);

        println!("{:?}", view);
    }

    let mut tera = compile_templates!(format!("{}/**/*.html", *THEME_PATH).as_str());
    tera.autoescape_on(vec![]);
    tera
}
