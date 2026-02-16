# 🚀 AeroFlow Quick Start Guide for Beginners

Welcome to AeroFlow! This guide will teach you how to run AeroFlow programs from your terminal.

---

## 📍 Step 1: Open Your Terminal

- Press `Win + R`, type `powershell`, and press Enter
- OR: Right-click in your project folder and select "Open in Terminal"

---

## 📂 Step 2: Navigate to the AeroFlow Directory

In your terminal, type:

```powershell
cd C:\Users\suman\Downloads\Aeroflow
```

**What this does:** Changes your current directory to the AeroFlow project folder.

---

## ▶️ Step 3: Run Your First AeroFlow Program

To run the demo file (`aeroflow-backend/main.aefl`), type:

```powershell
cargo run -p aeroflow-cli -- run --source aeroflow-backend/main.aefl
```

**Breaking down the command:**
- `cargo run` - Runs a Rust project
- `-p aeroflow-cli` - Specifies which package to run (the AeroFlow CLI)
- `--` - Separates cargo arguments from your program arguments
- `run --source aeroflow-backend/main.aefl` - Tells AeroFlow to run the specified `.aefl` file

**Expected output:**
```
🌀 AeroFlow Elite: Executing source: aeroflow-backend/main.aefl
🎯 Build Target: server | Platforms: []
🚀 Launching das runtime...
🔒 Running deterministic DAS loop...
✅ Execution complete.
```

---

## 📝 Step 4: Create and Run Your Own Program

### Create a new file
1. Create a file called `hello.aefl` in the main directory
2. Add this content:

```aeroflow
from http core

render { "Hello, AeroFlow! 🚀" }
render { "This is my first program!" }
```

### Run it
```powershell
cargo run -p aeroflow-cli -- run --source hello.aefl
```

---

## 🛠️ Common Commands

### Run a program
```powershell
cargo run -p aeroflow-cli -- run --source <your-file.aefl>
```

### Check if everything compiles correctly
```powershell
cargo check
```

### Build the release version (faster execution)
```powershell
cargo build --release -p aeroflow-cli
```

### Get help on available commands
```powershell
cargo run -p aeroflow-cli -- --help
```

---

## 💡 Tips for Beginners

1. **Always be in the right directory**: Before running commands, make sure you're in `C:\Users\suman\Downloads\Aeroflow`

2. **Check your location**: Type `pwd` to see where you are

3. **List files**: Type `ls` to see all files in the current directory

4. **Clear screen**: Type `cls` to clear your terminal

5. **Previous commands**: Press ↑ (up arrow) to cycle through previous commands

---

## 🎯 Quick Examples

### Example 1: Simple render
```aeroflow
render { "Welcome to AeroFlow!" }
```

### Example 2: Multiple spawns
```aeroflow
from http core
from db kv

spawn Database
spawn AuthService
spawn UserService

render { "🌀 All services started!" }
```

### Example 3: Run it
```powershell
cargo run -p aeroflow-cli -- run --source your-file.aefl
```

---

## ❓ What If Something Goes Wrong?

### Error: "cannot find file"
- Make sure the file path is correct
- Use `ls` to see available files

### Error: "cargo: command not found"
- You need to install Rust: https://rustup.rs/

### Error: Compilation failed
- Check your `.aefl` file syntax
- Make sure all `render` statements use braces: `render { "text" }`

---

## 🎓 Next Steps

1. ✅ Run the demo file (`aeroflow-backend/main.aefl`)
2. ✅ Create your own simple `.aefl` file
3. ✅ Experiment with different render statements
4. 📖 Read the full documentation in `docs/`

Happy coding with AeroFlow! 🚀
