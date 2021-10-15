use anyhow::{anyhow, Result};
use std::{fs, path};

pub fn file_name(path: &path::PathBuf) -> Result<String> {
    Ok(path
        .file_name()
        .ok_or(anyhow!("Failed to parse file name."))?
        .to_str()
        .ok_or(anyhow!("Failed to convert file name."))?
        .to_string())
}

pub fn create_dir(path: &path::PathBuf) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    Ok(())
}

pub fn clean_text(text: &str) -> Vec<String> {
    let lines = text.lines();

    lines
        .map(|x| x.chars().filter(|c| !c.is_whitespace()).collect())
        .filter(|x: &String| !x.is_empty())
        .collect()
}
