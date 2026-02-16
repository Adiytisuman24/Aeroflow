# 🏗️ AeroFlow Mobile LLVM Pipeline

This document details the high-performance compilation path for AeroFlow Mobile applications.

## 1. Compilation Flow
AeroFlow uses a specialized LLVM backend to transform high-level `.aefl` source into native machine code while preserving strict execution determinism.

```text
.aefl Source (UI + AI + Logic)
   ↓
Parser & AST Generator
   ↓
AeroFlow IR (Deterministic Bytecode)
   ↓
LLVM IR Generation (with deterministic memory layout)
   ↓
┌──────────────────────┬──────────────────────┐
│       Android        │         iOS          │
│   (Kotlin / NDK)     │    (Swift / LLVM)    │
└──────────┬───────────┴──────────┬───────────┘
           ↓                      ↓
     Mobile Binary          Mobile Binary
           +                      +
    Runtime Snapshot        Runtime Snapshot
```

## 2. Key Pillars of Mobile Determinism

### A. Deterministic Memory Layout
Unlike traditional mobile runtimes (JVM/Swift) which use non-deterministic GC or ARC, AeroFlow allocates memory in **Local Arenas**. This ensures:
- No GC pauses (Zero-GC).
- Identical memory addresses across different devices for the same logical tick.
- Stable pointers for time-travel state resumption.

### B. LLVM Optimization
LLVM serves as our Tier-1 backend, providing:
- **AOT Compilation**: Mobile apps start in microseconds using `.afs` snapshots.
- **Cross-Architecture Stability**: Bit-identical floating point operations across ARM (Mobile) and x86 (Server/Dev).

### C. Resource Embedding
All AI models (`agent` blocks) and UI assets are bundled into the final binary as deterministic resource chunks, accessible via the `from` package system.

## 3. Native Platform Integration
AeroFlow communicates with native mobile OS layers via a **Deterministic Bridge**:
- **User Input**: Captured as a message and injected into the DAS Scheduler at the next logical tick.
- **Sensors/Network**: Values are recorded in the replay log, allowing exactly repeatable sensor-driven simulations.
