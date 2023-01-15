use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Read},
    path::{Path, PathBuf},
};

fn get_folder_root() -> io::Result<PathBuf> {
    let mut path = env::current_exe()?;
    path.pop();
    path.push("proyects.txt");
    return Ok(path);
}

///Get proyects content if already exist, or create instead.
pub fn get_proyects_content() -> io::Result<String> {
    let path_to_proyects = get_folder_root()?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path_to_proyects.as_path())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    return Ok(content);
}

pub fn write_proyects(content: &str) -> io::Result<()> {
    let path_to_proyects = get_folder_root()?;
    fs::write(path_to_proyects, content)?;
    Ok(())
}

///Append new proyect to first line.
///
/// If the path already exists, then change the place to first line
pub fn append_to_first_proyect(url: &Path) -> io::Result<()> {
    let proyects_content = get_proyects_content()?;
    let proyect_paths: Vec<&str> = proyects_content.lines().collect();
    let path = match url.to_str() {
        Some(p) => p,
        None => return Err(io::Error::new(io::ErrorKind::Other, "Cannot parse url")),
    };

    if proyect_paths.contains(&path) {
        let proyects_text = proyect_paths
            .iter()
            .filter(|s| **s != path)
            .map(|s| *s)
            .collect::<Vec<&str>>()
            .join("\n");

        write_proyects(&format!("{}\n{}", path, proyects_text))?;
        return Ok(());
    }
    // If the new url not exist in the proyects saved:
    write_proyects(&format!("{}\n{}", path, proyect_paths.join("\n")))?;
    return Ok(());
}

pub fn delete_proyect(path: &Path) -> io::Result<()> {
    let binding = get_proyects_content()?;

    let filtered: Vec<String> = binding
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .filter(|p| **p != path.display().to_string().as_str())
        .map(|s| s.to_string())
        .collect();

    write_proyects(&filtered.join("\n"))?;

    Ok(())
}
