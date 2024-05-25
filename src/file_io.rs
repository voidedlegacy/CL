use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

fn file_size(file: &mut File) -> io::Result<u64> {
    let original_pos = file.seek(SeekFrom::Current(0))?;
    let size = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(original_pos))?;
    Ok(size)
}

fn file_contents(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut file = File::open(&path)?;
    let size = file_size(&mut file)?;

    let mut contents = String::with_capacity(size as usize);
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match file_contents("example.txt") {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

