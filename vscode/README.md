# AeroFlow VS Code Extension

First-class editor support for the AeroFlow programming language.

## Features

✨ **Syntax Highlighting** - Color-coded keywords, types, and constructs  
🔍 **Type Checking** - Real-time error detection  
📊 **Inline Diagnostics** - See errors as you type  
🎯 **Go-to Definition** - Navigate to function/actor definitions  
💡 **Auto-completion** - Smart suggestions for keywords, functions, and actors  
▶️ **Run/Build Commands** - Execute AeroFlow directly from VS Code

## Requirements

- **AeroFlow CLI** installed and in PATH
- **afls** (AeroFlow Language Server) available

### Install AeroFlow

```bash
# macOS/Linux
curl -fsSL https://get.aeroflow.dev/install.sh | sh

# Windows
iwr https://get.aeroflow.dev/install.ps1 -useb | iex
```

## Usage

1. Open any `.af` file
2. Start coding - syntax highlighting activates automatically
3. Press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS)
4. Type "AeroFlow" to see available commands:
   - **AeroFlow: Run** - Execute current file
   - **AeroFlow: Build** - Build executable
   - **AeroFlow: Snapshot** - Create production snapshot

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Run current file | `F5` |
| Build | `Ctrl+Shift+B` |

## Language Features

### Syntax Highlighting

The extension recognizes:
- Keywords: `fn`, `actor`, `on`, `state`, `emit`, `send`, `recv`
- Types: `Int`, `Float`, `Bool`, `String`, `List`, `Map`
- Control flow: `if`, `else`, `for`, `while`, `match`
- Effects: `effect`, `spawn`

### Code Completion

Type any keyword prefix to see suggestions:
- `fn` → function template
- `actor` → actor template
- `send(` → message send signature

### Error Detection

Errors appear inline with:
- 🔴 Red squiggles for syntax errors
- ⚠️ Yellow warnings for unused variables
- 💡 Hints for optimization opportunities

## Extension Settings

This extension contributes the following settings:

* `aeroflow.enableLSP`: Enable/disable language server
* `aeroflow.lspPath`: Custom path to `afls` binary
* `aeroflow.trace.server`: Trace LSP communication (for debugging)

## Known Issues

- Large files (>10k lines) may have delayed syntax highlighting
- Hot reload requires manual file save (auto-save not yet supported)

## Release Notes

### 1.0.0

Initial release:
- Syntax highlighting
- Basic LSP integration
- Run/Build commands

### 0.9.0

Beta preview release

## Contributing

Found a bug or want a feature? [Open an issue](https://github.com/aeroflow/vscode-aeroflow/issues).

## License

Apache 2.0

---

**Enjoy coding with AeroFlow!** 🚀
