# 🛠️ AeroFlow CLI — Full Specification

This document details the complete command-line interface for the AeroFlow Elite Engine.

## 1️⃣ Install AeroFlow

Install AeroFlow on Linux, macOS, or Windows:

```bash
# Clone the AeroFlow repo and build binaries
git clone https://github.com/Adiytisuman24/Aeroflow.git
cd Aeroflow
# If using make
make install
# Or manual build
cargo install --path cli
```

✅ **This installs:**
- `aeroflow` CLI
- Deterministic runtime (`das`)
- Mobile and WASM backends
- AeroFlow IDE (white/dark theme)
- Package manager (`afpm`)

---

## 2️⃣ Initialize a New Project

```bash
# Initialize a new AeroFlow project
aeroflow init myapp
cd myapp
```

**Generates project structure:**
```text
myapp/
 ├─ src/
 │   └─ main.aefl      # Main AeroFlow source
 ├─ snapshots/         # DAS runtime snapshots
 ├─ logs/              # Execution & replay logs
 ├─ ide/               # AeroFlow IDE config & cache
 ├─ aeroflow.toml      # Project configuration
 └─ packages/          # Installed modules
```

---

## 3️⃣ Build / Compile

Compile for Mobile, Web, or Server targets.

```bash
aeroflow build \
  --source ./src/main.aefl \
  --target mobile,server,wasm \
  --platform android,ios,web \
  --snapshot ./snapshots/main.afs \
  --ai
```

### Flags
| Flag | Purpose |
| :--- | :--- |
| `--source` | Path to `.aefl` source file. |
| `--target` | Build target: `mobile`, `server`, `wasm`. |
| `--platform` | Platforms: `android`, `ios`, `web`, `linux`, `windows`. |
| `--snapshot` | Path to save deterministic runtime snapshot (`.afs`). |
| `--ai` | Compile AI-native pipelines and tensor ops. |

---

## 4️⃣ Run / Execute

Execute the program with the Deterministic Actor Scheduler (DAS).

```bash
aeroflow run \
  --source ./src/main.aefl \
  --runtime das \
  --snapshot ./snapshots/main.afs \
  --log ./logs/execution.log \
  --replay \
  --ide ./ide
```

### Optional Flags
| Flag | Purpose |
| :--- | :--- |
| `--runtime` | Runtime engine: `das` (Deterministic Actor Scheduler). |
| `--log` | Path to execution log for replay/debugging. |
| `--replay` | Replay recorded execution logs. |
| `--ide` | Launch AeroFlow IDE with timeline & distributed state visualization. |
| `--dark-theme` | Launch IDE in dark mode. |
| `--light-theme` | Launch IDE in light mode. |
| `--distributed` | Enable multi-node distributed execution. |
| `--fast-mode` | Microsecond zero-cold-start execution. |
| `--verbose` | Show runtime logs in terminal. |

---

## 5️⃣ Package Manager — afpm

Install and manage AeroFlow modules.

```bash
# Install a package
aeroflow install

# (Future) Explicit package management
# afpm install ai.tensor
```

**Syntax in code:**
```ae
from ui.core view
from ai.tensor agent
from fintech.backtest
```

---

## 6️⃣ IDE Integration

Launch AeroFlow IDE with timeline, DAG, and distributed state visualization:

```bash
aeroflow ide \
  --file ./src/main.aefl \
  --dark-theme \
  --show-timeline \
  --show-distributed-state \
  --ai-debug
```

**IDE features:**
- Time-travel debugging
- Actor DAG visualization
- Distributed state snapshots
- Replay UI/network/AI events
- White/Dark mode

---

## 7️⃣ Distributed Simulation / Multiplayer / Blockchain

Run multi-node deterministic simulations:

```bash
aeroflow simulate \
  --source ./src/main.aefl \
  --nodes 5 \
  --distributed \
  --log ./logs/simulation.log \
  --replay
```

- `--nodes`: Number of simulated nodes.
- Deterministic scheduler ensures all nodes produce same state.
- Supports FinTech backtesting, multiplayer games, blockchain smart contracts.
- All events logged for reproducibility.

---

## 8️⃣ AI / ML Pipelines

Compile and run deterministic AI inference:

```bash
aeroflow run \
  --source ./src/main.aefl \
  --ai \
  --runtime das \
  --snapshot ./snapshots/ai.afs \
  --log ./logs/ai.log
```

**Supports:**
- `agent` blocks
- Deterministic tensor ops
- Replayable AI execution
- Cross-platform (mobile, web, server)

---

## 9️⃣ Replay & Time-Travel Debugging

```bash
aeroflow replay \
  --log ./logs/execution.log \
  --ide ./ide \
  --step 10ms \
  --fast-forward
```

- Step through deterministic execution.
- Replay network calls, UI events, threads, sensor input.
- Debug distributed systems & AI pipelines.
- Visualize timeline in IDE.

---

## 🔹 Example Full Command — All-in-One

```bash
aeroflow run \
  --source ./src/main.aefl \
  --target mobile,server,wasm \
  --platform android,ios,web \
  --runtime das \
  --snapshot ./snapshots/main.afs \
  --log ./logs/execution.log \
  --replay \
  --ide ./ide \
  --distributed \
  --ai \
  --fast-mode \
  --dark-theme
```

**What it does:**
1. Compiles `.aefl` source for all platforms (Android/iOS/Web/Server).
2. Launches DAS runtime.
3. Executes AI agents and tensor operations.
4. Runs distributed simulation (multi-node).
5. Saves snapshot (`.afs`) for zero cold start.
6. Launches AeroFlow IDE with timeline & state visualization.
7. Enables replayable deterministic logs.
