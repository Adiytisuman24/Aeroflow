# AeroFlow CLI - Command Reference

Complete guide to all AeroFlow command-line tools.

---

## Installation Verification

After installing AeroFlow, verify it's working:

```bash
aeroflow --version
# Output: aeroflow 1.0.0

aeroflow doctor
# Runs system diagnostics
```

---

## Commands

### `aeroflow init <project-name>`

**Initialize a new AeroFlow project.**

```bash
aeroflow init myapp
```

**Creates:**
```
myapp/
├── src/
│   └── main.af          # Entry point
├── assets/              # Static files
├── aeroflow.toml        # Project config
├── manifest.afm         # Runtime manifest
└── README.md
```

**Options:**
- `--template <name>` - Use a project template (e.g., `api`, `ai-worker`, `mobile`)

**Examples:**
```bash
# Basic project
aeroflow init hello

# API server template
aeroflow init myapi --template api

# AI worker template
aeroflow init ai-bot --template ai-worker
```

---

### `aeroflow run <file.af>`

**Execute an AeroFlow program (production mode).**

```bash
aeroflow run src/main.af
```

**How it works:**
1. Parses source code
2. Compiles to IR (Tier-0 bytecode)
3. Executes in scheduler
4. Shows execution time

**Output:**
```
🚀 Starting AeroFlow Runtime: src/main.af
✅ Execution finished in 1.23ms
```

**Options:**
- `--sandbox` - Run in isolated VM
- `--trace` - Enable execution tracing

**Examples:**
```bash
# Standard run
aeroflow run app.af

# Sandboxed execution
aeroflow run app.af --sandbox

# With tracing
aeroflow run app.af --trace
```

---

### `aeroflow dev <file.af>`

**Start development server with hot reload.**

```bash
aeroflow dev src/main.af
```

**Features:**
- File watcher enabled
- Auto-recompile on save
- State preserved across reloads
- Live error reporting

**Output:**
```
🔥 AeroFlow Dev Server (Hot Reload Active)
👀 Watching src/main.af...
🔄 Hot Reload: No changes detected.
```

**Options:**
- `--port <port>` - Specify server port (default: 8080)
- `--host <host>` - Bind address (default: 0.0.0.0)

**Examples:**
```bash
# Standard dev mode
aeroflow dev src/main.af

# Custom port
aeroflow dev src/main.af --port 3000

# Localhost only
aeroflow dev src/main.af --host 127.0.0.1
```

---

### `aeroflow build <file.af>`

**Build a portable executable artifact.**

```bash
aeroflow build src/main.af
```

**Output:**
- `dist/app.aefl` - Portable executable

**With Docker Sync (for large assets):**
```bash
aeroflow build src/main.af --docker-sync
```

**Output:**
- `dist/app.afs` - Snapshot file
- Docker image (assets layer only)

**How Docker Sync works:**
1. Large assets (models, datasets) → Docker image
2. Runtime logic → `.afs` snapshot
3. Deployment: Pull image + run snapshot
4. Result: Fast distribution + Fast execution

**Options:**
- `--docker-sync` - Enable Docker asset packaging
- `--output <path>` - Custom output directory

**Examples:**
```bash
# Standard build
aeroflow build app.af

# With Docker sync
aeroflow build app.af --docker-sync

# Custom output
aeroflow build app.af --output ./release
```

---

### `aeroflow snapshot <file.af>`

**Create AOT snapshot for ultra-fast deployment.**

```bash
aeroflow snapshot src/main.af
```

**Output:**
- `dist/app.afs` - Production snapshot

**What's included:**
- Compiled bytecode
- Hot native blocks (JIT cache)
- Memory layout map
- Permission manifest

**Cold start performance:**
- Standard: ~10ms
- Snapshot: **<10µs** ⚡

**Options:**
- `--optimize` - Apply AOT optimizations
- `--strip` - Remove debug symbols

**Examples:**
```bash
# Standard snapshot
aeroflow snapshot app.af

# Optimized production snapshot
aeroflow snapshot app.af --optimize --strip
```

---

### `aeroflow doctor`

**Diagnose system health and compatibility.**

```bash
aeroflow doctor
```

**Checks:**
- ✅ CPU features (AVX2, SIMD)
- ✅ Memory model (64-bit alignment)
- ✅ Docker bridge availability
- ✅ VM isolation support
- ✅ GPU availability (optional)

**Output:**
```
🩺 AeroFlow Doctor
✔ CPU Features: AVX2 detected
✔ Memory Model: 64-bit Little Endian
✔ Docker Bridge: Online
✔ VM Isolation: Ready
✅ System Healthy
```

**Options:**
- `--verbose` - Show detailed diagnostic info
- `--fix` - Attempt to fix common issues

---

## Advanced Usage

### Combining Commands

```bash
# Init + Dev in one go
aeroflow init myapp && cd myapp && aeroflow dev src/main.af

# Build + Snapshot  
aeroflow build app.af && aeroflow snapshot dist/app.aefl
```

### Environment Variables

```bash
# Set log level
export AEROFLOW_LOG=debug
aeroflow run app.af

# Custom cache directory
export AEROFLOW_CACHE=~/.cache/aeroflow
aeroflow build app.af
```

### Configuration File

`aeroflow.toml`:
```toml
[build]
target = "native"
optimize = true

[runtime]
threads = 4
sandbox = false

[dev]
port = 8080
hot_reload = true
```

---

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Compilation error |
| 2 | Runtime error |
| 101 | System check failed |

---

## Troubleshooting

### Command not found

```bash
# Verify installation
which aeroflow

# If not found, add to PATH
export PATH="$PATH:/usr/local/bin"
```

### Permission denied

```bash
# AeroFlow never needs root
# If you see this, something is wrong
aeroflow doctor --verbose
```

### Slow compilation

```bash
# Clear cache
rm -rf ~/.cache/aeroflow

# Rebuild
aeroflow build --fresh
```

---

## Getting Help

```bash
# General help
aeroflow --help

# Command-specific help
aeroflow build --help
aeroflow run --help
```

**Online Resources:**
- Documentation: https://docs.aeroflow.dev
- GitHub Issues: https://github.com/aeroflow/aeroflow/issues
- Discord: https://discord.gg/aeroflow

---

**Last updated: 2026-02-15**
