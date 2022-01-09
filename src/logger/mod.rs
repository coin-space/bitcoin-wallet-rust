use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::prelude::*;


pub fn path_exists(path: &str) -> bool {
    Path::new(path).is_file()
}

pub fn write(log: &str) -> std::io::Result<()> {
    let file_name = "wallet.log";

    if !path_exists(file_name) {
        File::create(file_name)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();

    file.write(log.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}
