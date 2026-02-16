# 🎓 BEGINNER'S GUIDE: Running AeroFlow

Welcome! This is your simple, step-by-step guide to running AeroFlow programs.

---

## ✅ Quick Start (3 Easy Steps!)

### 🎯 RECOMMENDED METHOD: Use VS Code Terminal

This is THE EASIEST way! VS Code automatically puts you in the right folder.

### 1️⃣ Open the Aeroflow Folder in VS Code

- **In VS Code:** Click `File` → `Open Folder`
- **Select:** `C:\Users\suman\Downloads\Aeroflow`
- **Click:** "Select Folder"

### 2️⃣ Open the Integrated Terminal

- **Press:** `Ctrl + ` ` (backtick key, below Esc)
- **Or:** Click `Terminal` → `New Terminal`
- ✅ **You're automatically in the Aeroflow folder!**

**Build the CLI (First time only):**
```powershell
cargo build -p aeroflow-cli
```

⏱️ **This takes about 30 seconds.** You only need to do this once!

---

### 📍 Alternative: Use External PowerShell (Not Recommended)

If you don't want to use VS Code terminal:

**Method 1:** File Explorer
- Go to: `C:\Users\suman\Downloads\Aeroflow`
- Hold `Shift` + Right-click in the folder
- Select "Open PowerShell window here"

**Method 2:** Navigation
- Open PowerShell
- Type: `cd C:\Users\suman\Downloads\Aeroflow`
- Press Enter

### 3️⃣ Run your AeroFlow program:

```powershell
.\target\debug\aeroflow-cli.exe run --source hello.aefl
```

**That's it!** 🎉

---

## 📋 What Each Part Means

Let's break down the command: `.\target\debug\aeroflow-cli.exe run --source hello.aefl`

- `.\target\debug\` = Where the program is located
- `aeroflow-cli.exe` = The AeroFlow program
- `run` = Command to run a file
- `--source hello.aefl` = Which file to run

---

## 🚀 Try These Examples

### Example 1: Run the demo file
```powershell
.\target\debug\aeroflow-cli.exe run --source hello.aefl
```

**You should see:**
```
🌀 AeroFlow Elite: Executing source: hello.aefl
🎯 Build Target: server | Platforms: []
🚀 Launching das runtime...
🔒 Running deterministic DAS loop...
✅ Execution complete.
```

### Example 2: Create your own file

**Step A:** Create a new file called `mytest.aefl` with this content:
```aeroflow
from http core

render { "My name is Suman!" }
render { "I'm learning AeroFlow! 🌟" }
```

**Step B:** Run it:
```powershell
.\target\debug\aeroflow-cli.exe run --source mytest.aefl
```

---

## 🛠️ Common Commands

### Run a file
```powershell
.\target\debug\aeroflow-cli.exe run --source YOUR-FILE.aefl
```

### Get help
```powershell
.\target\debug\aeroflow-cli.exe --help
```

### See what's in the folder
```powershell
ls
```

or

```powershell
dir
```

### Clear the screen
```powershell
cls
```

---

## 📝 Important Files in Your Project

```
Aeroflow/
├── hello.aefl                  ← Simple test file (TRY THIS FIRST!)
├── aeroflow-backend/
│   └── main.aefl               ← Backend demo
├── target/
│   └── debug/
│       └── aeroflow-cli.exe    ← The compiled program
├── QUICK_START.md              ← Detailed guide
└── BEGINNER_GUIDE.md           ← This file!
```

---

## 💡 Tips for Beginners

### Tip 1: Use the Easy Launcher (Already Created!)

I've already created a `run.bat` file for you! Instead of typing the long command:

```powershell
.\target\debug\aeroflow-cli.exe run --source hello.aefl
```

**Just type:**

```powershell
.\run.bat hello.aefl
```

**Much shorter and easier!** 🎉

### Tip 2: See your current location
```powershell
pwd
```

This shows where you are. You should see:
```
Path
----
C:\Users\suman\Downloads\Aeroflow
```

### Tip 3: Use arrow keys
- Press ↑ (up arrow) to show your last command
- Press ↓ (down arrow) to go forward through commands
- Press Tab to auto-complete file names

### Tip 4: If you get "command not found"
Make sure you're in the right folder! Run:
```powershell
cd C:\Users\suman\Downloads\Aeroflow
```

---

## ❌ Troubleshooting

### Problem: "aeroflow-cli.exe not found"

**Solution:** Build it first!
```powershell
cargo build -p aeroflow-cli
```

### Problem: "file not found" when running

**Solution:** Check the file exists:
```powershell
ls *.aefl
```

This shows all AeroFlow files. Make sure your file is there!

### Problem: "cargo: command not found"

**Solution:** Install Rust from https://rustup.rs/ and restart PowerShell

---

## 🎯 Your First Mission

1. ✅ Open PowerShell in the AeroFlow folder
2. ✅ Run: `cargo build -p aeroflow-cli` (first time only)
3. ✅ Run: `.\target\debug\aeroflow-cli.exe run --source hello.aefl`
4. ✅ Create your own `.aefl` file and run it!

---

## 🎉 Success Checklist

After running a program, you should see:
- ✅ `🌀 AeroFlow Elite: Executing source:...`
- ✅ `🚀 Launching das runtime...`
- ✅ `✅ Execution complete.`

If you see these, **congratulations!** You're running AeroFlow! 🎊

---

## 📚 Next Steps

Once you're comfortable:
1. Read `QUICK_START.md` for more advanced features
2. Explore files in the `examples/` folder
3. Check out `docs/` for complete documentation
4. Experiment with creating your own programs!

---

**Remember:** Programming is about experimenting and learning. Don't be afraid to try things!

Happy coding! 🚀
