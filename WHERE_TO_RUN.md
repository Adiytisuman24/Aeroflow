# 📍 Where Can I Run AeroFlow? (Beginner's Guide)

## 🎯 Quick Answer

**Currently:** You MUST be in the `Aeroflow` folder  
**After setup:** You can run it from ANYWHERE

---

## 🔍 Understanding the Problem

When you run this command:
```powershell
.\target\debug\aeroflow-cli.exe run --source hello.aefl
```

The `.` means "current folder". So this only works if you're IN the Aeroflow folder!

---

## ✅ METHOD 1: Always Run From Aeroflow Folder (EASY)

### In VS Code:
1. Open the `Aeroflow` folder in VS Code (File → Open Folder)
2. Press `` Ctrl + ` `` to open the integrated terminal
3. The terminal automatically opens in the Aeroflow folder!
4. Run your command:
   ```powershell
   .\target\debug\aeroflow-cli.exe run --source hello.aefl
   ```

### Why this works:
VS Code's terminal automatically starts in your project folder!

---

## ✅ METHOD 2: Run From ANYWHERE (ADVANCED)

### Option A: Use Full Paths

From any folder, you can run:
```powershell
C:\Users\suman\Downloads\Aeroflow\target\debug\aeroflow-cli.exe run --source C:\Users\suman\Downloads\Aeroflow\hello.aefl
```

**Example:** Even from `Desktop`, this works!

### Option B: Add to System PATH (Best for Advanced Users)

**Step 1:** Add AeroFlow to your PATH
```powershell
# Run this ONCE (in PowerShell as Administrator)
[Environment]::SetEnvironmentVariable(
    "Path",
    [Environment]::GetEnvironmentVariable("Path", "User") + ";C:\Users\suman\Downloads\Aeroflow\target\debug",
    "User"
)
```

**Step 2:** Restart PowerShell

**Step 3:** Now from ANYWHERE:
```powershell
aeroflow-cli run --source C:\Users\suman\Downloads\Aeroflow\hello.aefl
```

---

## 🖥️ Using VS Code Terminal (RECOMMENDED)

### ✅ The BEST Way for Beginners:

1. **Open VS Code**
2. **File → Open Folder** → Select `C:\Users\suman\Downloads\Aeroflow`
3. Press `` Ctrl + ` `` (backtick) to open terminal
4. You're automatically in the right folder!

### Check where you are:
```powershell
pwd
```

You should see:
```
Path
----
C:\Users\suman\Downloads\Aeroflow
```

### Now run your program:
```powershell
.\target\debug\aeroflow-cli.exe run --source hello.aefl
```

---

## 📊 Comparison

| Method | Difficulty | Where Can You Run? | Command Example |
|--------|-----------|-------------------|-----------------|
| **VS Code Terminal** | ⭐ Easy | In Aeroflow folder | `.\target\debug\aeroflow-cli.exe run --source hello.aefl` |
| **Full Paths** | ⭐⭐ Medium | Anywhere | `C:\Users\...\aeroflow-cli.exe run --source C:\Users\...\hello.aefl` |
| **System PATH** | ⭐⭐⭐ Advanced | Anywhere | `aeroflow-cli run --source C:\Users\...\hello.aefl` |

---

## 🎓 What Happens If You're in the Wrong Folder?

### Example of ERROR:
```powershell
# If you're in C:\Users\suman\Desktop
PS C:\Users\suman\Desktop> .\target\debug\aeroflow-cli.exe run --source hello.aefl
```

**Error:**
```
.\target\debug\aeroflow-cli.exe : The term '.\target\debug\aeroflow-cli.exe' is not recognized...
```

**Why?** Because there's no `target` folder in `Desktop`!

### ✅ Solution:
Navigate to the right folder first:
```powershell
cd C:\Users\suman\Downloads\Aeroflow
.\target\debug\aeroflow-cli.exe run --source hello.aefl
```

---

## 🎯 RECOMMENDED WORKFLOW (Beginners)

### **Use VS Code's Integrated Terminal:**

```
1. Open VS Code
   ↓
2. Open Folder (Ctrl+K Ctrl+O)
   → Select: C:\Users\suman\Downloads\Aeroflow
   ↓
3. Open Terminal (Ctrl+`)
   ↓
4. Run: .\target\debug\aeroflow-cli.exe run --source hello.aefl
   ↓
5. ✅ It works!
```

### **Why This is Best:**
- ✅ Always in the right folder
- ✅ Can edit files and run in one place
- ✅ No need to switch between windows
- ✅ Terminal automatically starts in project folder

---

## 🚀 VS Code Terminal Shortcuts

| Shortcut | Action |
|----------|--------|
| `` Ctrl + ` `` | Open/close terminal |
| `Ctrl+Shift+5` | Split terminal |
| `Ctrl+Shift+C` | Copy from terminal |
| `Ctrl+Shift+V` | Paste to terminal |

---

## 💡 Pro Tips

### Tip 1: Quick File Creation in VS Code
1. Right-click in Explorer panel
2. "New File" → `myprogram.aefl`
3. Write your code
4. Run in terminal below!

### Tip 2: Check Your Location
Always verify before running:
```powershell
pwd
```

### Tip 3: Navigate Quickly
```powershell
# Short form:
cd C:\Users\suman\Downloads\Aeroflow

# Even shorter (if already in Downloads):
cd Aeroflow
```

### Tip 4: Use Tab Completion
Type `.\tar` and press `Tab` → it completes to `.\target\`

---

## 📋 Quick Reference Card

### ✅ YOU ARE IN THE RIGHT PLACE IF:
```powershell
pwd
# Shows: C:\Users\suman\Downloads\Aeroflow
```

### ✅ RUN YOUR PROGRAM:
```powershell
.\target\debug\aeroflow-cli.exe run --source hello.aefl
```

### ❌ YOU ARE IN THE WRONG PLACE IF:
```powershell
pwd
# Shows: C:\Users\suman\Desktop  (or anything else)
```

### ✅ FIX IT:
```powershell
cd C:\Users\suman\Downloads\Aeroflow
```

---

## 🎉 Summary

**For Beginners (RECOMMENDED):**
- ✅ Open the `Aeroflow` folder in VS Code
- ✅ Use VS Code's integrated terminal (`` Ctrl + ` ``)
- ✅ You're automatically in the right place!
- ✅ Run: `.\target\debug\aeroflow-cli.exe run --source hello.aefl`

**For Advanced Users:**
- Add to PATH and run from anywhere
- Or use full paths

---

**Remember:** The easiest way is to use VS Code's terminal. It handles the folder location for you automatically! 🎯
