use super::*;
use notify::{
    DebouncedEvent,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch() -> notify::Result<()> {
    let (sender, receiver) = channel();
    let mut watcher: RecommendedWatcher = try!(Watcher::new(sender, Duration::from_secs(2)));

    try!(watcher.watch(Path::new(&*CONTENT_PATH), RecursiveMode::Recursive));

    loop {
        match receiver.recv() {
            Ok(event) => {
                match event {
                    DebouncedEvent::Chmod(path_buf) |
                    DebouncedEvent::Create(path_buf) |
                    DebouncedEvent::Remove(path_buf) |
                    DebouncedEvent::Write(path_buf) => {
                        on_content_change(path_buf);
                    },
                    DebouncedEvent::Rename(_, path_buf_dest) => {
                        on_content_change(path_buf_dest);
                    },
                    _ => {},
                }
            },
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

fn on_content_change(path: PathBuf) {
}
