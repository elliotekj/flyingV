use globset::GlobMatcher;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use super::*;
use tera::Tera;
use walkdir::WalkDir;

pub fn get() -> Tera {
    let mut tera = Tera::default();
    let theme_dir = PathBuf::from(&THEME_PATH.as_str());
    let theme_dir_walker = WalkDir::new(theme_dir).into_iter();

    for entry in theme_dir_walker {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();

        if path.is_file() {
            let tera_path_str = &path_str[THEME_PATH.len()+1..]; // +1 removes the leftover `/`
            if utils::is_hidden_file(&entry) || !utils::is_html_file(&entry) || tera_path_str.starts_with("views/") { continue; }

            let _ = tera.add_template_file(entry.path().to_owned(), Some(&tera_path_str.to_owned()));
        }
    }

    tera.autoescape_on(vec![]);
    tera
}

pub fn get_path_data() -> HashMap<String, GlobMatcher> {
    // key: the path in which Tera should look for the template
    // value: a glob ← any file in the contents that matches that glob should be rendered with the
    // template passed as the key.
    let mut path_data = HashMap::new();

    let views_dir = PathBuf::from(&THEME_PATH.as_str()).join("views");
    let tmp_dir = PathBuf::from(&THEME_PATH.as_str()).join("__tmp__");
    build_tmp_dir(&tmp_dir);

    let views_dir_walker = WalkDir::new(views_dir).into_iter();

    for entry in views_dir_walker {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if utils::is_hidden_file(&entry) || !utils::is_html_file(&entry) { continue; }

            let file = io::read(entry.path());
            let view = parser::view(file).unwrap();
            let path_stem = entry.path().file_stem().unwrap().to_str().unwrap();
            let tera_path_str = format!("{}/{}.html", &tmp_dir.to_str().unwrap()[THEME_PATH.len()+1..], path_stem);
            let tmp_path_str = format!("{}/{}.html", tmp_dir.to_str().unwrap(), path_stem);

            path_data.insert(tera_path_str.to_string(), view.target);
            io::simple_write(Path::new(&tmp_path_str), view.template);
        }
    }

    path_data
    // TODO: destroy_tmp_dir(&tmp_views_dir);
}

fn build_tmp_dir(path: &PathBuf) {
    let _ = fs::create_dir_all(path);
}

// fn destroy_tmp_dir(path: &PathBuf) {
//     let _ = fs::remove_dir_all(&path);
// }
