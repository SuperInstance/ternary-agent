# Ternary Agent

**Ternary Agent** provides the core agent types for the ternary ecosystem — every agent operates in one of three states (Avoid = -1, Explore = 0, Choose = +1), carrying memory, fitness scores, and behavioral metadata through an agent pool with fitness-based ranking.

## Why It Matters

Agent-based modeling frameworks need a foundational type that every higher-level system can build on. This crate defines that type: `Agent` with a `TernaryState` (Avoid/Explore/Choose), `AgentMemory` (short-term and long-term with decay), `Fitness` scoring, and an `AgentPool` with ranking. These three states map to the fundamental cognitive cycle: explore (gather information), choose (commit to an option), avoid (reject harmful options). This is the balanced ternary digit set {-1, 0, +1} applied to cognitive science.

## How It Works

### Ternary State Machine

```
Avoid (-1) ──→ Explore (0) ──→ Choose (+1) ──→ Avoid (-1)
```

Each state has distinct semantics:
- **Avoid** (-1): Agent rejects current option, retreats from threat
- **Explore** (0): Agent gathers information without commitment
- **Choose** (+1): Agent commits to a course of action

The `next()` method cycles: Avoid → Explore → Choose → Avoid. State transitions: **O(1)**.

### Agent Memory Model

Memory is split into short-term and long-term stores:

```
MemoryEntry { key, value, strength: f64 }

decay(clear_short_term, long_term_factor):
  if clear_short_term: short_term.clear()
  for entry in long_term: entry.strength *= long_term_factor
```

Short-term entries are O(1) to add, O(N) to clear. Long-term entries decay exponentially — strength approaches 0 as t → ∞ but never reaches it. Memory access: **O(N)** linear scan for key lookup.

### Agent Pool Ranking

The `AgentPool` tracks all agents with a `rank_by_fitness()` method:

```
rank_by_fitness() → Vec<(agent_id, fitness)>
  sorted descending by fitness
```

Ranking: **O(N log N)** for N agents (sort by fitness). Pool membership: **O(1)** via HashMap.

### Agent Communication

`AgentCommunication` provides typed message passing:

```
send(from, to, message) → Result
receive(agent_id) → Vec<Message>
```

Message routing: **O(1)** via HashMap of agent ID to mailbox.

### Fitness

Fitness is a simple `f64` score per agent, updated externally:

```
agent.fitness = evaluate_performance(agent)
```

Higher fitness = higher survival probability in evolutionary selection.

## Quick Start

```rust
use ternary_agent::{Agent, TernaryState, AgentPool};

let mut agent = Agent::new("alpha", TernaryState::Explore);
agent.memory.short_term.push(("observation".into(), "data".into(), 1.0));
agent.fitness = 0.85;

let mut pool = AgentPool::new();
pool.add(agent);
let ranked = pool.rank_by_fitness();
```

## API

| Type | Description |
|------|-------------|
| `TernaryState` | `Avoid (-1)`, `Explore (0)`, `Choose (+1)` with `to_trit()` and `next()` |
| `Agent` | Identity, state, memory, fitness, generation tracking |
| `AgentMemory` | Short-term (clearable) and long-term (decaying) stores |
| `MemoryEntry` | key, value, strength triple |
| `AgentPool` | Collection with `rank_by_fitness()` |
| `AgentCommunication` | Message passing between agents |
| `Fitness` | `f64` score type |

## Architecture Notes

Ternary Agent is the foundational type crate for the entire SuperInstance ternary ecosystem. In γ + η = C, the three states directly encode the equation: Choose (+1) = γ (growth), Avoid (-1) = η (avoidance), Explore (0) = neutral equilibrium state. Every other ternary crate builds on these types.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for the ternary agent architecture.

## References

1. Holland, J. H. (1992). *Adaptation in Natural and Artificial Systems*, 2nd ed. MIT Press.
2. Russell, S. & Norvig, P. (2021). *AI: A Modern Approach*, 4th ed. Pearson. Chapter 2: Intelligent Agents.
3. Knuth, D. E. (1981). *The Art of Computer Programming, Vol. 2*, 2nd ed. Section 4.1: Balanced Ternary Notation.

## License

MIT
