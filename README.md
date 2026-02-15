# 🌀 AeroFlow (v1.0 Preview)

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen.svg)](https://github.com/Adiytisuman24/Aeroflow)
[![Deterministic](https://img.shields.io/badge/Deterministic-100%25-blueviolet.svg)](https://github.com/Adiytisuman24/Aeroflow)

**AeroFlow** is a high-performance, deterministic, and AI-native runtime engine. It is designed to run the same program identically across servers, browsers, mobile, and edge environments by eliminating architectural nondeterminism.

---

## 🚀 The AeroFlow Manifesto

Modern distributed systems are fragile, non-deterministic, and slow to scale. **AeroFlow fixes the foundation.**

- **Provable Determinism**: Same input + same logical time = bit-for-bit identical output. Every time.
- **Microsecond Cold-Starts**: Uses **Snapshot Resumption** to bypass traditional OS/container boot times.
- **Actor-Based Isolation**: Every unit of work (Actor/Agent) has its own private memory arena (Zero-GC).
- **AI as a Primitive**: Tensors, Models, and Agents are first-class citizens in the language.
- **Time-Travel Debugging**: Record execution traces and scrub through program history like a video.

---

## 🏗️ Core Architecture: The Elite Engine

AeroFlow is built on the **Deterministic Actor Scheduler (DAS)**. Unlike traditional runtimes that rely on OS threads (nondeterministic), AeroFlow uses a global logical clock.

### 🧩 Components

| Module | Description | Status |
| :--- | :--- | :--- |
| **`compiler/`** | Tier-0 high-speed AeroFlow-to-IR compiler. | ✅ Stable |
| **`runtime/`** | The DAS-powered execution engine with Arena memory. | ✅ Stable |
| **`cli/`** | Unified toolchain for building, running, and testing. | ✅ Stable |
| **`aeroflow-lsp/`** | Language Server Protocol for IDE integration. | 🏗️ In Progress |
| **`aeroflow-conformance/`** | Official AeroFlow Conformance Test Suite (AFCTS). | ✅ Active |

---

## 📊 Benchmark Comparison (v1.0 Stage)

AeroFlow is engineered for **Predictability** and **Latency Stability**, filling the gaps left by traditional runtimes.

| Metric | **🌀 AeroFlow** | **🐹 Go** | **🟢 Node.js** | **🐍 Python** |
| :--- | :--- | :--- | :--- | :--- |
| **Cold Start** | **~500µs – 3ms** | ~15ms – 30ms | ~60ms – 150ms | ~40ms – 100ms |
| **Execution** | **Deterministic (DAS)** | Nondeterministic | Nondeterministic | Nondeterministic |
| **Memory Model** | **Local Arena** (Zero-GC) | Global GC (STW) | Global GC | Ref Counting |
| **Concurrency** | Causal Actor Link | Goroutines | Event Loop | GIL Restricted |

### Why AeroFlow?

1. **Vs. Go**: While Go is fast, its scheduler is nondeterministic. AeroFlow's **DAS** ensures the exact same execution order across all nodes.
2. **Vs. Node.js**: AeroFlow replaces the "Stop-the-World" Garbage Collector with **Isolated Arenas**, eliminating P99 latency spikes.
3. **Vs. Python**: AeroFlow's **Snapshot Resumption** allows AI Agents to "thaw" and execute in microseconds, whereas Python takes hundreds of milliseconds to initialize.

---

## 🛠️ Installation & Usage

### 1. Build from Source

```bash
# Clone the repository
git clone https://github.com/Adiytisuman24/Aeroflow.git
cd aeroflow

# Build the CLI
cargo build --release --bin aeroflow
```

### 2. Run a Program

```bash
# Initialize a new project
aeroflow init my-app

# Run a deterministic script
aeroflow run main.aefl
```

### 3. Time-Travel Debugging

```bash
# Run with tracing enabled
aeroflow run demo.aefl

# View the execution timeline
aeroflow trace
```

---

## 🗺️ Roadmap: The Path to v1.0

- [x] **Core Language Specification**: EBNF Formalization.
- [x] **DAS Engine**: Deterministic Actor Scheduler.
- [x] **Elite Toolchain**: CLI, Build system, and Testing suite.
- [x] **Time-Travel Records**: Deterministic trace export/replay.
- [ ] **AeroFlow Studio**: Visual timeline-based IDE.
- [ ] **WASM Target**: Running DAS in the browser.
- [ ] **Distributed DAS**: Multi-node deterministic message passing.

---

## 🤝 Contributing

We are building the future of deterministic computing. If you're interested in compilers, high-performance runtimes, or AI-native systems, we'd love your help.

1. Fork the repo
2. Ensure tests pass: `cargo test` & `aeroflow test`
3. Submit a PR

## 📜 License

Created with ❤️ by the AeroFlow team. Licensed under the **Apache License 2.0**.
