use BUILD_PATH;
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

pub fn write(path: &Path, contents: String) {
    let build_path = PathBuf::from(&BUILD_PATH.as_str());
    let mut file_path = PathBuf::from(build_path.join(path));
    file_path.set_extension("html");

    if let Some(dirpath) = path.parent() {
        let _ = fs::create_dir_all(build_path.join(&dirpath));
    };

    let file = File::create(file_path).expect("Unable to create the file");
    let mut file = BufWriter::new(file);
    file.write_all(contents.as_bytes()).expect("Unable to write data to file");
}
