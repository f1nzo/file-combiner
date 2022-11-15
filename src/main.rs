use std::{fs::{self, File}, io};
use std::time::Instant;

fn main() {
    let now = Instant::now();
    match combine() {
        Ok(_) => {
            let elapsed = now.elapsed();
            println!("Done in {:?}", elapsed);
        },
        Err(e) => println!("Error: {}", e),
    }
}

fn combine() -> Result<(), std::io::Error> {
    let mut output = File::create("combined.txt")?;
    for entry in fs::read_dir("input")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let mut input = File::open(path)?;
            io::copy(&mut input, &mut output)?;
        }
    }
    Ok(())
}