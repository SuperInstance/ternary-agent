# ternary-agent: Core agent types for the ternary ecosystem

Agent struct with ternary state (avoid/explore/choose), agent pools with fitness tracking, memory with short-term and long-term decay, behavior strategies, and inter-agent message passing.

## Why This Exists

Multi-agent systems need a foundational type that every other component can build on. Rather than using continuous numeric states, ternary-agent uses three discrete states — Avoid, Explore, Choose — mapped to balanced ternary digits {-1, 0, +1}. This makes agent decisions composable with ternary math libraries across the ecosystem.

## Core Concepts

- **TernaryState** — Three states an agent can be in: Avoid (−1), Explore (0), Choose (+1). Maps directly to a balanced ternary digit (trit).
- **Agent** — A single entity with an ID, current state, fitness score, memory, and optional behavior strategy.
- **AgentMemory** — Split into short-term (recent observations, easily cleared) and long-term (persistent but decays over time). Each memory has a strength value that decreases on decay.
- **AgentBehavior** — Wraps a `Strategy` that decides the next state given a score. Strategies are trait objects so you can plug in any decision logic.
- **AgentPool** — A collection of agents with fitness-based ranking and batch operations.
- **AgentCommunication** — A message bus where agents send and receive messages by ID.

## Quick Start

```toml
[dependencies]
ternary-agent = "0.1"
```

```rust
use ternary_agent::*;

// Create an agent with a threshold behavior
let strategy = Box::new(ThresholdStrategy::new(-0.5, 0.5));
let behavior = AgentBehavior::new("threshold", strategy);
let mut agent = Agent::new(1).with_behavior(behavior);

// The agent starts in Explore state
assert_eq!(agent.state, TernaryState::Explore);

// Tick with a high score → agent chooses
agent.tick(0.8);
assert_eq!(agent.state, TernaryState::Choose);

// Observe and recall from memory
agent.observe("door", "locked");
let (value, strength) = agent.recall("door").unwrap();
assert_eq!(value, "locked");
```

## API Overview

| Type | Description |
|------|-------------|
| `TernaryState` | Enum: Avoid, Explore, Choose. Converts to/from trit values. |
| `Agent` | Core entity: state, fitness, memory, optional behavior. |
| `AgentMemory` | Short-term + long-term storage with strength decay. |
| `MemoryEntry` | A single key-value pair with a strength float. |
| `AgentBehavior` | Wraps a Strategy, tracks decision count. |
| `Strategy` (trait) | Decide next state from current state + score. |
| `ThresholdStrategy` | Built-in: Avoid below lower threshold, Choose above upper, Explore in between. |
| `AgentPool` | HashMap of agents with fitness ranking. |
| `AgentCommunication` | Message bus keyed by agent ID. |
| `AgentMessage` | from, to, tag, payload. |

## How It Works

Agents don't run autonomously. External code calls `agent.tick(score)` which runs the attached strategy (if any) to determine the new state, then updates fitness. Memory is separate from behavior — you can observe, commit short-term to long-term, recall, and decay independently.

The `AgentPool` stores agents in a `HashMap<u64, Agent>` and provides ranked access sorted by fitness descending. The `AgentCommunication` bus is a simple `HashMap<u64, Vec<AgentMessage>>` — send pushes to inbox, receive drains it.

Memory decay multiplies long-term entry strengths by a factor and prunes below a minimum. Short-term can be cleared wholesale. This models a simple forgetting curve.

## Known Limitations

- Memory recall returns the first match — if duplicate keys exist in both short-term and long-term, short-term wins but duplicates within the same list are not handled.
- AgentCommunication is not thread-safe — it's a single-threaded message bus. For concurrent agents, wrap it in a `Mutex`.
- AgentPool ranking uses floating-point comparison which can produce inconsistent ordering for near-equal fitness values.
- No persistence — all state is in-memory. Serialize snapshots yourself.

## Use Cases

- **Game AI NPCs** — each NPC is an Agent with Avoid (flee), Explore (wander), Choose (attack) states driven by a threat score.
- **Robotic controller** — agents represent subsystems that avoid obstacles, explore paths, or choose a route based on sensor readings.
- **Trading signal pipeline** — each strategy is an agent that avoids (sell), explores (hold), or chooses (buy) based on a signal score.
- **Load balancer** — agents represent servers; pool ranking routes traffic to the fittest (least loaded).

## Ecosystem Context

`ternary-agent` is a foundational crate. It depends on nothing outside `std`. Other ternary crates like `ternary-room` (multi-agent environments), `ternary-ensign` (specialist agents), and `ternary-world` (simulation engine) build on top of the types defined here.

## See Also

- **ternary-room** — Room abstraction for multi-agent environments
- **ternary-cortex** — Cognitive architecture for ternary agents
- **ternary-fitness** — Fitness landscape analysis for ternary strategies
- **ternary-memory** — Memory systems for ternary agents
- **ternary-cell** — Cellular computing with ternary state machines
- **ternary-popgen** — Population genetics for ternary agents
- **ternary-bus** — Message bus for inter-agent communication

## License

MIT
