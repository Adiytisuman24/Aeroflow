# ⚡ AeroFlow Installation Guide

## 📥 1. Clone the Repository
```bash
git clone https://github.com/Adiytisuman24/Aeroflow.git
cd Aeroflow
```

## 🛠️ 2. Build from Source (Rust)
AeroFlow is built with Rust. Ensure you have `cargo` installed.

```bash
cargo install --path cli
```

## 🏗️ 3. Verify Installation
```bash
aeroflow --version
aeroflow --help
```

## 📦 4. Initialize a Project
```bash
aeroflow init my-app
cd my-app
aeroflow run main.aefl
```

## 🌐 5. Advanced Usage
For advanced multi-node simulation and mobile compilation, refer to the [CLI Reference](./CLI_REFERENCE.md).
