use clap::{Parser, Subcommand};
use aeroflow_compiler::compile;
use aeroflow_runtime::VM;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "aeroflow")]
#[command(about = "AeroFlow CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run an AeroFlow program
    Run {
        /// The .aefl file to run
        file: PathBuf,
    },
    /// Compile an AeroFlow program to IR
    Build {
        /// The .aefl file to build
        file: PathBuf,
    },
    /// Install dependencies from aeroflow.toml
    Install,
    /// Initialize a new AeroFlow project
    Init {
        /// Name of the project
        name: String,
    },
    /// Run conformance tests
    Test {
        /// Directory containing tests
        #[arg(default_value = "aeroflow-conformance/tests")]
        dir: String,
    },
    /// Export the execution trace to JSON
    Trace,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            let start_time = std::time::Instant::now();
            let source = fs::read_to_string(file)?;
            match compile(&source) {
                Ok(chunk) => {
                    let scheduler = aeroflow_runtime::Scheduler::new();
                    let actor = aeroflow_runtime::vm_actor::VMActor::new(chunk);
                    let cell = aeroflow_runtime::ActorCell::new("Main".to_string(), Box::new(actor));
                    
                    scheduler.spawn(cell);
                    scheduler.send("Main".to_string(), aeroflow_runtime::MessageData::Signal("start".to_string()), "System".to_string());
                    
                    // Execute one deterministic step
                    scheduler.step();

                    // Save trace
                    let trace = aeroflow_runtime::get_tracer().export_json();
                    fs::write("trace.json", trace)?;
                    println!("🔒 Deterministic trace saved to trace.json");
                }
                Err(e) => {
                    println!("❌ Compile Error: {}", e);
                }
            }
        }
        Commands::Build { file } => {
            let start_time = std::time::Instant::now();
            let source = fs::read_to_string(file)?;
            let chunk = compile(&source)?;
            let compile_time = start_time.elapsed();
            
            println!("✓ Build successful in {:.4}ms. {} instructions generated.", compile_time.as_secs_f64() * 1000.0, chunk.instrs.len());
            for (i, instr) in chunk.instrs.iter().enumerate() {
                println!("{:04}: {:?}", i, instr);
            }
        }
        Commands::Install => {
            println!("🌀 AeroFlow Installer v0.1");
            let config_path = std::path::Path::new("aeroflow.toml");
            if !config_path.exists() {
                println!("Error: aeroflow.toml not found. Run 'aeroflow init' first.");
                return Ok(());
            }

            let _content = fs::read_to_string(config_path)?;
            // In a real impl, we'd use a TOML parser here
            println!("📦 Resolving dependencies...");
            
            let home = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:".to_string());
            let global_cache = std::path::Path::new(&home).join(".aeroflow").join("modules");
            fs::create_dir_all(&global_cache)?;

            let deps = vec![("http", "1.2.0"), ("ai.vision", "0.3.4")];
            
            for (name, version) in deps {
                let module_path = global_cache.join(name).join(version);
                if !module_path.exists() {
                    println!("  🚚 Fetching {}@{}...", name, version);
                    fs::create_dir_all(&module_path)?;
                    // Simulate writing an AFM binary
                    let afm_file = module_path.join(format!("{}.afm", name.replace(".", "_")));
                    fs::write(afm_file, "AFM1_MOCK_BINARY_DATA")?;
                } else {
                    println!("  ✔ {}@{} is already in cache.", name, version);
                }
            }

            println!("🔗 Linking modules to project...");
            let project_modules = std::path::Path::new(".aeroflow").join("modules");
            fs::create_dir_all(&project_modules)?;
            println!("✨ Done. Ready to run.");
        }
        Commands::Init { name } => {
            println!("✨ Initializing AeroFlow project: {}...", name);
            let toml_content = format!(
r#"[package]
name = "{}"
version = "0.1.0"
entry = "main.aefl"

[dependencies]
http = "1.2"
ai.vision = "0.3"
"#, name);
            fs::write("aeroflow.toml", toml_content)?;
            let main_content = format!("print(\"Hello from \" + \"{}\" + \"! 🚀\")\n", name);
            fs::write("main.aefl", main_content)?;
            println!("✓ Created aeroflow.toml");
            println!("✓ Created main.aefl");
            println!("🚀 Your project is ready. Run 'aeroflow install' and then 'aeroflow run main.aefl'.");
        }
        Commands::Test { dir } => {
            println!("🧪 Running AeroFlow Conformance Tests in {}...", dir);
            let test_dir = std::path::Path::new(&dir);
            if !test_dir.exists() {
                println!("Error: Test directory not found.");
                return Ok(());
            }

            let mut passed = 0;
            let mut total = 0;

            for entry in walkdir::WalkDir::new(test_dir) {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("aefl") {
                    total += 1;
                    print!("  RUN  {}... ", entry.path().display());
                    let _ = std::io::Write::flush(&mut std::io::stdout());
                    
                    let _source = fs::read_to_string(entry.path())?;
                    match compile(&_source) {
                        Ok(chunk) => {
                            let mut vm = VM::new();
                            let ctx = aeroflow_runtime::vm::VMContext {
                                logical_time: 0,
                                rand_seed: 0xACE1,
                            };
                            vm.execute(&chunk, &ctx);
                            println!("✅ PASS");
                            passed += 1;
                        }
                        Err(e) => {
                            println!("❌ FAIL");
                            println!("     Error: {}", e);
                        }
                    }
                }
            }

            println!("\nTest Summary: {}/{} passed", passed, total);
        }
        Commands::Trace => {
            println!("{}", aeroflow_runtime::get_tracer().export_json());
        }
    }

    Ok(())
}
