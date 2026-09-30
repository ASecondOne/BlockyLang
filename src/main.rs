use std::fs::self;
use colored::Colorize;

use blocky_lang::{executers::executer::executer, psychoparser::psycho_parser::attempt_psycho_parse, symbolresolver::symbol_resolver::resolve_psycho_blocks};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "init" => handle_init(),
            "run" => handle_run(args),
            _ => println!("{}", format!("Unknown Command: {}", arg).red()),
        }
    }
}

fn handle_init() {
    if !already_exists() {
        let blocky_toml = std::env::current_dir().unwrap().join("blocky.toml");
        let src_dir = std::env::current_dir().unwrap().join("src/");
        let premade_file = std::env::current_dir().unwrap().join("src/main.block");

        // //! currently does nothing
        let _ = fs::write(blocky_toml, "[unimportes]\n\n[imports]"); 

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

fn handle_run(args: Vec<String>) {
    if already_exists() {
        // //! Currently takes every file and treats them as one file
        // //! This might defiantly lead to problems with variables and function

        let mut debug = false;

        if let Some(arg) = args.get(2) {
            match arg.as_str() {
                "--debug" => debug = true,
                _ => {
                    println!("{}", format!("Unkown argument {} for run", arg).red());
                    std::process::exit(1);
                }
            }
        }
        
        let mut f_contents = Vec::new();
        
        let src_dir = std::env::current_dir().unwrap().join("src/");

        for file in fs::read_dir(src_dir).unwrap() {
            let path = file.unwrap().path();

            if path.extension().is_some_and(|ext| ext == "block") {
                let contents = fs::read_to_string(&path).unwrap();

                f_contents.push(contents);
            }
        }

        let out = match attempt_psycho_parse(f_contents) {
            Ok(out) => out,
            Err(()) => std::process::exit(1),
        };


        if debug {
            println!("\n|----------------PsyhoAST-----------------|\n");

            println!("{:#?}", out);

            println!("\n|----------------ResolvedAST-----------------|\n");
        }

        let resolved_out = match resolve_psycho_blocks(out) {
            Ok(out) => out,
            Err(()) => std::process::exit(1),
        };

        if debug {
            println!("{:#?}", resolved_out);

            println!("\n|----------------Output-----------------|\n");
        }

        executer(resolved_out);

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
