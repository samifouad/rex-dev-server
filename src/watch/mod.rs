use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn writes(watch_path: &str, port: &str) {
    println!("hello from secondary thread");
    println!("port number is: {}", port);

    // channel to receive filesystem events
    let (sender, receiver) = channel();

    // create a watcher object
    let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();

    // path to be watch
    watcher.watch(watch_path, RecursiveMode::Recursive).unwrap();

    loop {
        match receiver.recv() {
            Ok(event) => match event {
                DebouncedEvent::NoticeWrite(path) => println!("write: {:?}", path),
                DebouncedEvent::NoticeRemove(_) => todo!(),
                DebouncedEvent::Create(_) => todo!(),
                DebouncedEvent::Write(_) => todo!(),
                DebouncedEvent::Chmod(_) => todo!(),
                DebouncedEvent::Remove(_) => todo!(),
                DebouncedEvent::Rename(_, _) => todo!(),
                DebouncedEvent::Rescan => todo!(),
                DebouncedEvent::Error(_, _) => todo!(),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
