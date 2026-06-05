# ternary-agent

**Core agent types for the ternary ecosystem**

[![ternary](https://img.shields.io/badge/ecosystem-ternary-blue)](https://github.com/orgs/SuperInstance/repositories?q=ternary)
[![tests](https://img.shields.io/badge/tests-23-green)]()

## Overview

Core agent types for the ternary ecosystem.

Every agent operates in one of three states — Avoid, Explore, or Choose —
borrowed from the ternary digit set {-1, 0, +1}. An `Agent` carries memory,
fitness, and behavior. An `AgentPool` tracks a collection of agents with
fitness-based ranking. `AgentCommunication` provides message passing.

## Architecture

- **`MemoryEntry`** — core data structure
- **`AgentMemory`** — core data structure
- **`ThresholdStrategy`** — core data structure
- **`AgentBehavior`** — core data structure
- **`AgentMessage`** — core data structure
- **`AgentCommunication`** — core data structure
- **`Agent`** — core data structure
- **`AgentPool`** — core data structure
- **`TernaryState`** — state enumeration

### Traits

- **`Strategy`** — shared behavior contract

### Key Functions

- `to_trit()`
- `from_trit()`
- `next()`
- `new()`
- `observe()`
- `commit()`
- `recall()`
- `decay()`
- `len()`
- `is_empty()`
- ... and 25 more

## Why Ternary?

The balanced ternary system {-1, 0, +1} (also known as Z₃) is the mathematically optimal discrete encoding:
- **More expressive than binary**: three states capture positive, neutral, and negative
- **Natural for decisions**: accept/reject/abstain, buy/hold/sell, agree/disagree/neutral
- **Self-balancing**: the 0 state acts as a universal screen, preventing pathological lock-in
- **Z₃ cyclic dynamics**: rock-paper-scissors is the only natural coordination mechanism

## Stats

| Metric | Value |
|--------|-------|
| Lines of Rust | 642 |
| Test count | 23 |
| Public types | 9 |
| Public functions | 35 |

## Ecosystem

This crate is part of the **[SuperInstance Ternary Fleet](https://github.com/orgs/SuperInstance/repositories?q=ternary)**:

- **[ternary-core](https://github.com/SuperInstance/ternary-core)** — shared traits and Z₃ arithmetic
- **[ternary-grid](https://github.com/SuperInstance/ternary-grid)** — spatial grid with {-1, 0, +1} cells
- **[ternary-graph](https://github.com/SuperInstance/ternary-graph)** — ternary-weighted graph algorithms
- **[ternary-automata](https://github.com/SuperInstance/ternary-automata)** — three-state cellular automata
- **[ternary-compiler](https://github.com/SuperInstance/ternary-compiler)** — expression compiler and optimizer

200+ crates. 4,300+ tests. One pattern.

## Research Context

The ternary approach connects to several active research areas:
- **Ternary Neural Networks** (TNNs): weights constrained to {-1, 0, +1} for efficient inference
- **Huawei's ternary chip**: 7nm ternary silicon with 60% less power consumption
- **Active inference**: free energy minimization naturally maps to ternary action selection
- **Cyclic dominance**: RPS dynamics maintain biodiversity in spatial ecology
- **Z₃ group theory**: the only algebraic group on three elements is cyclic addition mod 3

## Usage

```toml
[dependencies]
ternary-agent = "0.1.0"
```

```rust
use ternary_agent;
```

## License

MIT
