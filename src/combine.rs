// function to combine all files in a directory into one file mulithreaded

use std::fs::{self, File};
use std::io;
use std::time::Instant;
use std::thread;
use std::sync::mpsc;

fn main() {
    let now = Instant::now();

    let (tx, rx) = mpsc::channel();

    for entry in fs::read_dir("input").unwrap() {
        let tx = tx.clone();
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            thread::spawn(move || {
                let mut input = File::open(path).unwrap();
                let mut buffer = Vec::new();
                input.read_to_end(&mut buffer).unwrap();
                tx.send(buffer).unwrap();
            });
        }
    }

    let mut output = File::create("combined.txt").unwrap();
    for buffer in rx {
        output.write_all(&buffer).unwrap();
    }

    let elapsed = now.elapsed();
    println!("Done in {:?}", elapsed);
}