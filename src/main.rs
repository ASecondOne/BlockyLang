use blocky_lang::cli::create_new_project;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if let Some(arg) = args.get(1) {
        match arg.as_str() {
            "new" => {
                let _ = create_new_project(args.get(2)).is_err_and(|err| {
                    println!("{err}");
                    true
                });
            },
            _ => println!("Unknown Command")  
        }
    }

    
}
