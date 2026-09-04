use std::fs::{self, read_dir};

use blocky_lang::psycho_parser::attempt_psycho_parse;
use colored::Colorize;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "init" => handle_init(),
            "run" => handle_run(),
            _ => println!("{}", format!("Unknown Command: {}", arg).red()),
        }
    }
}

fn handle_init() {
    if !already_exists() {
        let blocky_toml = std::env::current_dir().unwrap().join("blocky.toml");
        let src_dir = std::env::current_dir().unwrap().join("src/");
        let premade_file = std::env::current_dir().unwrap().join("src/main.block");

        let _ = fs::write(blocky_toml, "[unimports]\n\n[imports]\nimport ALL from i_core");

        let _ = fs::create_dir_all(src_dir);

        let _ = fs::write(premade_file, 
r#"<execute>
    println "Hello Block";
</execute>"#);

        println!("{}", format!("Project made in: {}", std::env::current_dir().unwrap().display()));

        return;
    }

    println!("{}", format!("Project already exists").red())
}

fn handle_run() {
    if already_exists() {
        // //! Currently takes every file and treats them as one file
        // //! This might defiantly lead to problems with variables and function
        
        let mut f_contents = Vec::new();
        
        let src_dir = std::env::current_dir().unwrap().join("src/");

        for file in fs::read_dir(src_dir).unwrap() {
            let path = file.unwrap().path();

            if path.extension().is_some_and(|ext| ext == "block") {
                let contents = fs::read_to_string(&path).unwrap();

                f_contents.push(contents);
            }
        }

        attempt_psycho_parse(f_contents);

        return;
    }

    println!("{}", format!("Project not found").red())
}

fn already_exists() -> bool {

    let dir = std::env::current_dir().unwrap();
    
    if dir.join("blocky.toml").exists() {
        return true;
    }

    false
}