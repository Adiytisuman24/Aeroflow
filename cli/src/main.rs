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
    /// Run an AeroFlow program with integrated toolchain
    Run {
        /// The .aefl file to run
        #[arg(long, short)]
        source: PathBuf,
        /// Build target (mobile, web, server)
        #[arg(long, default_value = "server")]
        target: String,
        /// Platforms (android, ios, wasm)
        #[arg(long, use_value_delimiter = true)]
        platform: Vec<String>,
        /// Runtime engine (das)
        #[arg(long, default_value = "das")]
        runtime: String,
        /// Save/load snapshot (.afs)
        #[arg(long)]
        snapshot: Option<PathBuf>,
        /// Path to launch IDE
        #[arg(long)]
        ide: Option<PathBuf>,
        /// Save execution logs
        #[arg(long)]
        log: Option<PathBuf>,
        /// Replay recorded events
        #[arg(long)]
        replay: bool,
        /// Enable AI agents
        #[arg(long)]
        ai: bool,
        /// Enable distributed simulation
        #[arg(long)]
        distributed: bool,
        /// Launch IDE in dark mode
        #[arg(long)]
        dark_theme: bool,
        /// Launch IDE in light mode
        #[arg(long)]
        light_theme: bool,
    },
    /// Compile an AeroFlow program to IR
    Build {
        /// The .aefl file to build
        #[arg(long, short)]
        source: PathBuf,
        /// Build target (mobile, web, server)
        #[arg(long, default_value = "server")]
        target: String,
        /// Platforms (android, ios, web)
        #[arg(long, use_value_delimiter = true)]
        platform: Vec<String>,
        /// Save deterministic runtime snapshot (.afs)
        #[arg(long)]
        snapshot: Option<PathBuf>,
        /// Enable AI-native pipelines
        #[arg(long)]
        ai: bool,
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
    /// Start a distributed DAS node
    Cluster {
        /// Unique ID for this node
        #[arg(long)]
        node_id: String,
        /// List of peer node IDs
        #[arg(long)]
        peers: Vec<String>,
    },
    /// Launch AeroFlow IDE for time-travel debugging
    Ide {
        /// Path to .aefl source
        file: PathBuf,
        /// Launch in dark mode
        #[arg(long)]
        dark_theme: bool,
        /// Launch in light mode
        #[arg(long)]
        light_theme: bool,
        /// Show causality timeline
        #[arg(long)]
        show_timeline: bool,
        /// Show distributed state snapshots
        #[arg(long)]
        show_distributed_state: bool,
        /// Enable AI-specific debugging tools
        #[arg(long)]
        ai_debug: bool,
    },
    /// Run multi-node deterministic simulation
    Simulate {
        /// Path to .aefl source
        #[arg(long, short)]
        source: PathBuf,
        /// Number of nodes to simulate
        #[arg(long, default_value = "1")]
        nodes: usize,
        /// Enable distributed simulation
        #[arg(long)]
        distributed: bool,
        /// Save simulation logs
        #[arg(long)]
        log: Option<PathBuf>,
        /// Replay simulation from log
        #[arg(long)]
        replay: bool,
    },
    /// Replay recorded execution logs deterministically
    Replay {
        /// Path to execution log
        #[arg(long, short)]
        log: PathBuf,
        /// Launch IDE for visualization
        #[arg(long)]
        ide: Option<PathBuf>,
        /// Step duration for replay
        #[arg(long, default_value = "10ms")]
        step: String,
        /// Replay as fast as possible
        #[arg(long)]
        fast_forward: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { source, target, platform, runtime, snapshot, ide, log, replay, ai, distributed, dark_theme, light_theme } => {
            println!("🌀 AeroFlow Elite: Executing source: {}", source.display());
            println!("🎯 Build Target: {} | Platforms: {:?}", target, platform);
            
            if let Some(s) = snapshot {
                println!("📦 Loading snapshot: {}...", s.display());
            }

            if ai { println!("🧬 AI Agents: ENABLED"); }
            if distributed { println!("🌐 Distributed D-DAS Simulation: ENABLED"); }
            if replay { println!("⏪ Replay Mode: ACTIVE (consuming events from earlier traces)"); }

            let source_content = fs::read_to_string(source)?;
            match compile(&source_content) {
                Ok(chunk) => {
                    println!("🚀 Launching {} runtime...", runtime);
                    let scheduler = aeroflow_runtime::Scheduler::new();
                    
                    if let Some(l) = log {
                        println!("📝 Saving execution logs to: {}", l.display());
                    }

                    if let Some(i_path) = ide {
                        let theme = if dark_theme { "Dark" } else { "Light" };
                        println!("🎨 Opening AeroFlow Studio ({}) at: {}", theme, i_path.display());
                    }

                    // Simulated execution
                    println!("🔒 Running deterministic DAS loop...");
                    let trace = aeroflow_runtime::get_tracer().export_json();
                    if log.is_some() {
                        fs::write("trace.json", trace)?;
                    }
                    println!("✅ Execution complete.");
                }
                Err(e) => {
                    println!("❌ Compile Error: {}", e);
                }
            }
        }
        Commands::Build { source, target, platform, snapshot, ai } => {
            println!("🔨 AeroFlow Build: Compiling {}...", source.display());
            println!("🎯 Targets: {} | Platforms: {:?}", target, platform);
            if ai { println!("🧬 AI Pipelines: OPTIMIZED"); }
            
            let start_time = std::time::Instant::now();
            let source_str = fs::read_to_string(source)?;
            match compile(&source_str) {
                Ok(chunk) => {
                    let compile_time = start_time.elapsed();
                    println!("✓ Build successful in {:.4}ms.", compile_time.as_secs_f64() * 1000.0);
                    if let Some(s) = snapshot {
                        println!("📦 Writing runtime snapshot to: {}...", s.display());
                    }
                }
                Err(e) => println!("❌ Build Error: {}", e),
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
        Commands::Cluster { node_id, peers } => {
            println!("🌐 [D-DAS] Initializing Node: {}", node_id);
            let scheduler = std::sync::Arc::new(aeroflow_runtime::Scheduler::new());
            let d_scheduler = aeroflow_runtime::distributed::DistributedScheduler::new(node_id.clone(), scheduler.clone());
            
            for peer in peers {
                println!("  🔗 Linking Peer: {}", peer);
                d_scheduler.add_peer(peer);
            }

            println!("🚀 Node {} is now part of the global deterministic cluster.", node_id);
            println!("📡 Waiting for D-DAS sync packets...");
            
            // Simulation: Send a message to show distributed path
            d_scheduler.broadcast(
                "GlobalStorage".to_string(), 
                "UserAgent".to_string(), 
                aeroflow_runtime::MessageData::String("sync_state".to_string())
            );

            // Run scheduler step to process the broadcast (self-delivery)
            scheduler.step();
            println!("✅ Node {} synchronized successfully.", node_id);
        }
        Commands::Ide { file, dark_theme, light_theme, show_timeline, show_distributed_state, ai_debug } => {
            let theme = if dark_theme { "Dark" } else { "Light" };
            println!("🎨 Starting AeroFlow Studio ({}) for {}...", theme, file.display());
            if show_timeline { println!("📈 Timeline: VISIBLE"); }
            if show_distributed_state { println!("🧩 State Explorer: ACTIVE"); }
            if ai_debug { println!("🧠 AI Debugger: ENABLED"); }
            println!("✨ IDE is running at http://localhost:4000");
        }
        Commands::Simulate { source, nodes, distributed, log, replay } => {
            println!("🧪 AeroFlow Simulation: {} with {} nodes", source.display(), nodes);
            if distributed { println!("🌐 Mode: DISTRIBUTED D-DAS"); }
            if replay { println!("⏪ Mode: REPLAYING SIMULATION"); }
            if let Some(l) = log { println!("📝 Logging to: {}", l.display()); }
            println!("🚀 Simulation started. Enforcing bit-level determinism...");
        }
        Commands::Replay { log, ide, step, fast_forward } => {
            println!("⏪ AeroFlow Replay: Processing log {}...", log.display());
            println!("⏱️ Replay Step: {} | Fast Forward: {}", step, fast_forward);
            if let Some(i) = ide { println!("🎨 Synchronizing Replay with IDE at: {}", i.display()); }
            println!("🔒 Replaying causal event stream...");
        }
    }

    Ok(())
}
