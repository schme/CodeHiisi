use std::env;
use std::path::PathBuf;

pub fn get_project_path() -> Result<PathBuf, std::io::Error> {
    env::current_dir()
}

pub fn get_asset_path() -> Result<PathBuf, std::io::Error> {
    let project_path = get_project_path()?;
    Ok(project_path.join("assets"))
}
