use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Read},
    path::{Path, PathBuf},
};

fn get_folder_root() -> io::Result<PathBuf> {
    let mut path = env::current_exe()?;
    path.pop();
    path.push("projects.txt");
    return Ok(path);
}

///Get projects content if already exist, or create instead.
pub fn get_projects_content() -> io::Result<String> {
    let path_to_projects = get_folder_root()?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path_to_projects.as_path())?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    return Ok(content);
}

pub fn write_projects(content: &str) -> io::Result<()> {
    let path_to_projects = get_folder_root()?;
    fs::write(path_to_projects, content)?;
    Ok(())
}

///Append new project to first line.
///
/// If the path already exists, then change the place to first line
pub fn append_to_first_project(url: &Path) -> io::Result<()> {
    let projects_content = get_projects_content()?;
    let project_paths: Vec<&str> = projects_content.lines().collect();
    let path = match url.to_str() {
        Some(p) => p,
        None => return Err(io::Error::new(io::ErrorKind::Other, "Cannot parse url")),
    };

    if project_paths.contains(&path) {
        let projects_text = project_paths
            .iter()
            .filter(|s| **s != path)
            .map(|s| *s)
            .collect::<Vec<&str>>()
            .join("\n");

        write_projects(&format!("{}\n{}", path, projects_text))?;
        return Ok(());
    }
    // If the new url not exist in the projects saved:
    write_projects(&format!("{}\n{}", path, project_paths.join("\n")))?;
    return Ok(());
}

pub fn delete_project(path: &Path) -> io::Result<()> {
    let binding = get_projects_content()?;

    let filtered: Vec<String> = binding
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .filter(|p| **p != path.display().to_string().as_str())
        .map(|s| s.to_string())
        .collect();

    write_projects(&filtered.join("\n"))?;

    Ok(())
}
