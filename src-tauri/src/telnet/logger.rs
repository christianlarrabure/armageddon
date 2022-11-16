use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn log(string: &String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("log.txt")
        .unwrap();
    file.write_all(&string.as_bytes())?;
    Ok(())
}
