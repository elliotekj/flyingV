use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use super::*;
use tera::Tera;
use walkdir::WalkDir;

pub fn get_tera() -> Result<Tera, ()> {
    let mut tera = Tera::default();
    let theme_dir = PathBuf::from(&THEME_PATH.as_str());
    let theme_dir_walker = WalkDir::new(theme_dir).into_iter();

    for entry in theme_dir_walker {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let entry_path_buf = entry_path.to_path_buf();
        let entry_path_str = entry_path.to_str().unwrap().to_string();

        if entry_path.is_file() {
            let tera_path_str = &entry_path_str[THEME_PATH.len()+1..]; // +1 removes the leftover `/`
            if utils::is_hidden_file(&entry_path_buf) || !utils::is_html_file(&entry_path_buf) || tera_path_str.starts_with("views/") { continue; }

            let _ = tera.add_template_file(entry_path.to_owned(), Some(&tera_path_str.to_owned()));
        }
    }

    tera.autoescape_on(vec![]);
    Ok(tera)
}

pub fn get_path() -> String {
    format!("{}/views", &THEME_PATH.as_str())
}

pub fn get_tmp_path() -> String {
    format!("{}/__tmp__", &THEME_PATH.as_str())
}

pub fn get_data() -> HashMap<String, View> {
    // key: the path in which Tera should look for the view
    // value: a glob ← any file in the contents path that matches that glob
    // should be rendered with the view passed as the key.
    let mut path_data = HashMap::new();

    let views_dir = PathBuf::from(&*VIEWS_PATH);
    let tmp_views_dir = PathBuf::from(&*VIEWS_TMP_PATH);
    build_tmp_views_dir();

    let views_dir_walker = WalkDir::new(views_dir).into_iter();

    for entry in views_dir_walker {
        let entry = entry.unwrap();
        let entry_path = entry.path();
        let entry_path_buf = entry_path.to_path_buf();

        if entry_path.is_file() {
            if utils::is_hidden_file(&entry_path_buf) || !utils::is_html_file(&entry_path_buf) { continue; }

            let file = io::read(entry_path);
            let view = parser::view(file).unwrap();
            let path_stem = entry_path.file_stem().unwrap().to_str().unwrap();
            let tera_path_str = format!("{}/{}.html", &tmp_views_dir.to_str().unwrap()[THEME_PATH.len()+1..], path_stem);
            let tmp_path_str = format!("{}/{}.html", tmp_views_dir.to_str().unwrap(), path_stem);

            io::simple_write(Path::new(&tmp_path_str), view.html.as_str());
            path_data.insert(tera_path_str.to_string(), view);
        }
    }

    path_data
}

fn build_tmp_views_dir() {
    let tmp_views_dir = PathBuf::from(&*VIEWS_TMP_PATH);
    let _ = fs::create_dir_all(tmp_views_dir);
}

pub fn destroy_tmp_views_dir() {
    let tmp_views_dir = PathBuf::from(&*VIEWS_TMP_PATH);
    let _ = fs::remove_dir_all(tmp_views_dir);
}
