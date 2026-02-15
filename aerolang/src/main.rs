mod ast;
mod parser;

use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.contains(&"--demo".to_string()) {
        aerolang::demo::main();
        return;
    }

    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "examples/hello.aero"
    };

    println!("📜 Reading AeroLang source: {}...", file_path);
    
    match fs::read_to_string(file_path) {
        Ok(src) => {
            let ast = parser::parse_program(&src);
            println!("📦 Parsed AST:");
            println!("{:#?}", ast);
        },
        Err(_) => {
            println!("Usage: aerolang <file.aero>");
            println!("       aerolang --demo");
        }
    }
}
