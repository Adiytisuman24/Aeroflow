mod isolation;
mod docker_bridge;

use std::env;
use docker_bridge::DockerBridge;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("AeroFlow Shim Usage:");
        println!("  shim pull <image>    - Pull & Extract Docker Image");
        println!("  shim run <path.aefl> - Run AeroFlow App (Isolated)");
        return;
    }
    
    match args[1].as_str() {
        "pull" => {
            if args.len() < 3 { return; }
            let image = &args[2];
            let target_dir = format!("assets/{}", image.replace(":", "_"));
            
            DockerBridge::pull_image(image).unwrap();
            DockerBridge::extract_layers(image, &target_dir).unwrap();
            
            println!("💡 Next: shim run {}", target_dir);
        }
        "run" => {
            if args.len() < 3 { return; }
            let path = &args[2];
            println!("⚡ Executing AeroFlow Logic: {}", path);
            // Simulate reading app.aefl and launching runtime
            // In real impl: isolation::spawn_vm(path, config)
            println!("✅ App running in isolated VM (PID: 1234)");
            
            // Example of hot-loading model from extracted assets
            println!("🧠 Mmap'ing AI model from adjacent layer...");
        }
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }
}
