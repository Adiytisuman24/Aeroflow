# 🏁 Getting Started with AeroFlow

Welcome to the **AeroFlow Elite Engine**, the future of high-performance, deterministic distributed computing.

## 📥 1. Installation

AeroFlow is built with Rust. You can install it on **Linux**, **macOS**, or **Windows**.

### Method A: Install Script (Recommended)
```bash
curl -fsSL https://registry.aeroflow.dev/install.sh | sh
```

### Method B: Build from Source
```bash
git clone https://github.com/Adiytisuman24/Aeroflow.git
cd Aeroflow
cargo install --path cli
```

---

## 🏗️ 2. Create Your First Project

AeroFlow projects are structured for scale. Initialize a new project with the CLI:

```bash
aeroflow init my-app
cd my-app
```

This creates:
- `src/main.aefl`: Your entry point.
- `aeroflow.toml`: Project configuration.

---

## ✍️ 3. Write Deterministic Code

Edit `src/main.aefl`:

```ae
from ui.core view
from ai.tensor agent

screen VerifyIdentity {
    let username: string = ""
    render {
        Text {"Please enter your username:"}
        Input {bind: username}
        Button {"Verify", onClick: AuthAgent.check(username)}
    }
}

agent AuthAgent {
    model "llama3-small"
    on check(user) {
        render {model.run("Is " + user + " a valid user?")}
    }
}

fn main() {
    render {
        VerifyIdentity {}
    }
}
```

---

## 🚀 4. Run & Simulate

### Run Locally (DAS Runtime)
Execute your app with the Deterministic Actor Scheduler:

```bash
aeroflow run --source src/main.aefl
```

### Run with AI & Distributed Simulation
Enable powerful features like AI agents and multi-node replay:

```bash
aeroflow run \
  --source src/main.aefl \
  --ai \
  --distributed \
  --log execution.log
```

---

## 🎨 5. Time-Travel Debugging (IDE)

Open the **AeroFlow Studio** to visually inspect your execution timeline:

```bash
aeroflow ide --file src/main.aefl --dark-theme
```

- Scrub through history.
- Inspect actor memory.
- View distributed message flows.

---

## 📱 6. Build for Production

Compile your app for mobile devices or the web:

```bash
aeroflow build \
  --source src/main.aefl \
  --target mobile \
  --platform android,ios \
  --snapshot production.afs
```

This generates optimized native binaries linked against the LLVM backend.

---

## 📚 Next Steps
- [Full CLI Reference](./CLI_REFERENCE.md)
- [Architecture Deep Dive](./ARCHITECTURE.md)
- [Mobile LLVM Pipeline](./MOBILE_LLVM_PIPELINE.md)
