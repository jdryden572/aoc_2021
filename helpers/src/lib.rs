use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(path.as_ref())?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub fn read_lines_panicky<P: AsRef<Path>>(path: P) -> impl Iterator<Item = String> {
    read_lines(path)
        .expect("Failed to open file")
        .map(|r| r.expect("Failed to read line"))
}
