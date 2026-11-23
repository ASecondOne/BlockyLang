use std::{fs, path::PathBuf};

use crate::parser::orchestrator::orchestrator;

pub fn create_new_project(path: Option<&String>) -> Result<(), String> {

    let path = if path.is_some() {
        PathBuf::from(path.unwrap())
    } else {
        PathBuf::from(".")
    };

    let _ = fs::create_dir_all(&path);

    let source_dir = path.clone().join(PathBuf::from("src"));
    let config_file = path.clone().join("blocky.yaml");

    if config_file.exists() {
        return Err("Project Already exists".to_string());
    }
    
    let _ = fs::write(config_file, "");

    let _ = fs::create_dir_all(&source_dir);

    let main_file = source_dir.join(PathBuf::from("main.block"));

    let _ = fs::write(main_file, "");

    Ok(())
}

pub fn run_project(path: Option<&String>) -> Result<(), String> {

    let path = if path.is_some() {
        PathBuf::from(path.unwrap())
    } else {
        PathBuf::from(".")
    };

    let _ = fs::create_dir_all(&path);

    let source_dir = path.clone().join(PathBuf::from("src"));
    let config_file = path.clone().join("blocky.yaml");

    if config_file.exists() {
        return Err("Project Already exists".to_string());
    }

    orchestrator(source_dir);

    Ok(())
}