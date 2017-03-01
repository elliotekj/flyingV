use globset::GlobMatcher;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use super::*;
use tera::Tera;
use walkdir::WalkDir;

pub fn get() -> (HashMap<String, GlobMatcher>, Tera) {
    let views_dir = PathBuf::from(&THEME_PATH.as_str()).join("views");
    let tmp_dir = PathBuf::from(&THEME_PATH.as_str()).join("__tmp__");
    build_tmp_dir(&tmp_dir);

    let mut view_templates = HashMap::new();
    let views_dir_walker = WalkDir::new(views_dir).into_iter();

    for entry in views_dir_walker {
        let entry = entry.unwrap();
        if utils::is_dotfile(&entry) || !utils::is_html_file(&entry) { continue; }

        let file = io::read(entry.path());
        let view = parser::view(file).unwrap();
        let tmp_path_str = format!("{}/{}.html",
                                   tmp_dir.to_str().unwrap(),
                                   entry.path().file_stem().unwrap().to_str().unwrap()
        );

        view_templates.insert(tmp_path_str.to_string(), view.target);
        io::simple_write(&Path::new(&tmp_path_str), view.template);
    }

    let mut tera = compile_templates!(format!("{}/[!views]/*.html", tmp_dir.to_str().unwrap()).as_str());
    tera.autoescape_on(vec![]);

    // destroy_tmp_dir(&tmp_views_dir);

    (view_templates, tera)
}

fn build_tmp_dir(path: &PathBuf) {
    let _ = fs::create_dir_all(path);
}

// fn destroy_tmp_dir(path: &PathBuf) {
//     let _ = fs::remove_dir_all(&path);
// }
