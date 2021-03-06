use std::fs::{self, File};
use std::io::{Read, BufReader, Write, BufWriter};
use std::path::{Path, PathBuf};

pub fn read(path: &Path) -> String {
    let mut contents = String::new();
    let file = File::open(&path).expect("Couldn't open file");
    let mut buffer = BufReader::new(file);
    buffer.read_to_string(&mut contents).expect("Couldn't read file");

    contents
}

pub fn write_page(tmp_build_path: &PathBuf, url: &str, contents: String) {
    let mut file_path_string = format!("{}{}", tmp_build_path.to_str().unwrap(), url);

    if url == "/" {
        let _ = fs::create_dir_all(Path::new(&file_path_string));
        file_path_string.push_str("index.html");
    } else {
        let _ = fs::create_dir_all(Path::new(&file_path_string));
        file_path_string.push_str("/index.html");
    }

    let file = File::create(Path::new(&file_path_string)).expect("Unable to create the file");
    let mut file = BufWriter::new(file);
    file.write_all(contents.as_bytes()).expect("Unable to write data to file");
}

pub fn simple_write(path: &Path, contents: &str) {
    let file = File::create(path).expect("Unable to create the file");
    let mut file = BufWriter::new(file);
    file.write_all(contents.as_bytes()).expect("Unable to write data to file");
}
