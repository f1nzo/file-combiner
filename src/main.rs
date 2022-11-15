use std::{fs::{self, File}, io};
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let mut output = File::create("combined.txt").unwrap();
    for entry in fs::read_dir("input").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let mut input = File::open(path).unwrap();
            io::copy(&mut input, &mut output).unwrap();
        }
    }

    let elapsed = now.elapsed();
    println!("Done in {:?}", elapsed);
}
