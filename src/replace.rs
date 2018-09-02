use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

pub fn replace_in_file(
    file_path: &Path,
    to_be_replaced: &str,
    replace_with: &str,
) -> io::Result<()> {
    let file_contents = read_file(file_path)?;
    let new_file_contents = file_contents.replace(to_be_replaced, replace_with);
    write_file(file_path, &new_file_contents)?;
    Ok(())
}

fn read_file(file_path: &Path) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    Ok(file_contents)
}

fn write_file(file_path: &Path, contents: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write(contents.as_bytes())?;
    Ok(())
}
