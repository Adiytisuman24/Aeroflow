# 🌀 AeroFlow Core Architecture

Deep dive into the Seven-Layer Elite Engine Flow.

## Layer 1: AeroFlow Source (.aefl)
Unified single-file programming model for:
- **UI**: Declarative `screen` and `render` blocks.
- **AI**: Agent-based model orchestration.
- **Distributed**: Multi-node state and timelines.

## Layer 2: Parser & AST
Translates syntax into a **Causal AST**. Every node is verified for deterministic evaluation order. Nested blocks (e.g., UI inside an Agent handler) are supported.

## Layer 3: Compiler (LLVM)
High-performance backend generating:
- **Mobile**: Android/iOS native code.
- **Web**: WASM benchmarks and edge nodes.
- **Snapshot (.afs)**: Frozen memory images for instant resumption.

## Layer 4: DAS Runtime
The **Deterministic Actor Scheduler (DAS)** is the heartbeat of AeroFlow. It ensures that every actor processes messages in a bit-identical global order.

## Layer 5: Distributed Simulation
The base layer for:
- **Multiplayer Games**: Total state sync across players.
- **FinTech**: Backtesting trading bots against historical tick-by-tick logs.
- **Blockchain**: Secure, reproducible smart contract execution.

## Layer 6: Platform Runtime
Native engines that interpret the `RenderUI`, `RenderTimeline`, and `RenderState` instructions.
- **Render Engine**: Updates UI frames.
- **Sensor Replay**: Feeds recorded gyro/gps data back into the DAS for debugging.

## Layer 7: IDE & Visualization (AeroFlow Studio)
Real-time causality visualization:
- **Timeline View**: Historical DAG of every event.
- **Snapshot Explorer**: Inspect actor memory at any logical tick.
- **Themes**: Professional Dark and High-Contrast Light.
