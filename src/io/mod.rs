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
    // Get the path components:
    let build_path_str = &BUILD_PATH.as_str();
    let file_path_str = &path.to_str().unwrap()[CONTENT_PATH.len()..];

    // Build the dir tree for the file:
    let file_parent = Path::new(file_path_str).parent().unwrap();
    let file_stem = path.file_stem().unwrap();
    let file_tree: PathBuf;

    // Write the dir tree for the file:
    if file_parent == Path::new("/") && file_stem == "index" {
        let file_tree_str = format!("{}{}/index.html", build_path_str, file_parent.to_str().unwrap());
        file_tree = PathBuf::from(&file_tree_str);
        let _ = fs::create_dir_all(Path::new(build_path_str));
    } else {
        let file_tree_str = format!("{}{}/{}/index.html", build_path_str, file_parent.to_str().unwrap(), file_stem.to_str().unwrap());
        file_tree = PathBuf::from(&file_tree_str);
        let _ = fs::create_dir_all(file_tree.parent().unwrap());
    }

    // Write the file:
    let file = File::create(file_tree).expect("Unable to create the file");
    let mut file = BufWriter::new(file);
    file.write_all(contents.as_bytes()).expect("Unable to write data to file");
}

pub fn simple_write(path: &Path, contents: String) {
    let file = File::create(path).expect("Unable to create the file");
    let mut file = BufWriter::new(file);
    file.write_all(contents.as_bytes()).expect("Unable to write data to file");
}
