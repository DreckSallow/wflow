use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Read},
    path::{Path, PathBuf},
};

/// This function get the file content of path passed to function
/// If the file already exist, then get content
/// otherwise, create the file and return empty String
pub fn get_content_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Write content in the file passed as path to function
pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

pub fn get_folder_program() -> io::Result<PathBuf> {
    let mut path = env::current_exe()?;
    path.pop();
    return Ok(path);
}
