use std::fs::{self, File};
use std::io::{Read, BufReader, Write, BufWriter};
use std::path::{Path, PathBuf};
use super::*;

pub fn read(path: &Path) -> String {
    let mut contents = String::new();
    let file = File::open(&path).expect("Couldn't open file");
    let mut buffer = BufReader::new(file);
    buffer.read_to_string(&mut contents).expect("Couldn't read file");

    contents
}

pub fn write(path: &Path, contents: String) {
    let build_path_str = &BUILD_PATH.as_str();
    let file_path_str = &path.to_str().unwrap()[*&CONTENT_PATH.len()..];

    let final_path_str = format!("{}{}", build_path_str, file_path_str);
    let mut final_path = PathBuf::from(final_path_str);
    final_path.set_extension("html");

    if let Some(file_tree) = Path::new(file_path_str).parent() {
        let file_tree_str = format!("{}{}", build_path_str, file_tree.to_str().unwrap());
        let file_tree = Path::new(&file_tree_str);

        let _ = fs::create_dir_all(file_tree);
    };

    let file = File::create(final_path).expect("Unable to create the file");
    let mut file = BufWriter::new(file);
    file.write_all(contents.as_bytes()).expect("Unable to write data to file");
}
