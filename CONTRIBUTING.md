# Contributing to AeroFlow

Thank you for your interest in helping build the most deterministic runtime on the planet! 🌀

## 🧪 Development Workflow

### 1. Build Requirements
- Rust (Stable 1.70+)
- Cargo

### 2. Testing
AeroFlow uses a two-tier testing system:
- **Unit/Integration Tests**: Standard Rust tests.
  ```bash
  cargo test --workspace
  ```
- **Conformance Tests (AFCTS)**: Language-level tests.
  ```bash
  cargo run -- test
  ```

## 🏗️ Architectural Guidelines
- **No Nondeterminism**: Avoid `std::time::Instant`, `rand::thread_rng`, or any OS-level timing in the `runtime/` and `compiler/` crates. Use the provided DAS context.
- **Arena Memory**: Use the per-actor memory arenas for performance critical allocations.
- **Lock-Free**: Prefer message passing via DAS over shared-state mutexes where possible.

## 🚀 Submitting Changes
1. Create a descriptive branch: `feature/new-instruction` or `fix/parser-bug`.
2. Follow the EBNF spec in `docs/GRAMMAR.ebnf`.
3. Provide a test case in `aeroflow-conformance/tests`.
4. Submit a Pull Request!

---

AeroFlow aims for **Elite Efficiency**. Every line of code should be deterministic, fast, and secure.
