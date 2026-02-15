use aeroflow_compiler::compile;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: afc <file.aefl>");
        return;
    }

    let source = fs::read_to_string(&args[1]).expect("Unable to read file");
    match compile(&source) {
        Ok(chunk) => {
            println!("Compiled successfully. IR Generated.");
            for (i, instr) in chunk.instrs.iter().enumerate() {
                println!("{:04}: {:?}", i, instr);
            }
        }
        Err(e) => eprintln!("Compilation Error: {}", e),
    }
}
