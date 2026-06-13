# Ternary Agent

Core agent types for the **ternary ecosystem** — a framework where every agent operates in one of three states drawn from balanced ternary {-1, 0, +1}: **Avoid**, **Explore**, or **Choose**. Provides agent memory (short-term + long-term with decay), threshold-based strategies, fitness tracking, agent pools with ranking, and inter-agent message passing.

## Why It Matters

Agent frameworks typically model behavior as continuous-valued functions or finite state machines with dozens of states. The ternary approach constrains agents to exactly three behavioral modes, mapped to the balanced ternary digit set:

| Trit | State | Semantics |
|------|-------|-----------|
| -1 | Avoid | Retreat, reject, diverge |
| 0 | Explore | Gather information, remain neutral |
| +1 | Choose | Commit, select, converge |

This constraint is a feature, not a limitation. It forces crisp decision boundaries and enables population-level analysis: you can compute the **γ (signed charge)** of an entire agent pool with a single sum, and track how it evolves over time. The three-state model also maps directly to Z₃ cyclic dominance (rock-paper-scissors dynamics), which is the natural coordination mechanism for ternary multi-agent systems.

## How It Works

### Ternary State Machine

Each agent holds a `TernaryState` that cycles: Avoid → Explore → Choose → Avoid. The state is updated by a `Strategy` — a trait that takes the current state plus a numeric score and returns the next state:

$$s_{t+1} = \text{strategy}(s_t, \text{score}_t)$$

The default `ThresholdStrategy` uses upper and lower bounds:

$$s_{t+1} = \begin{cases} \text{Choose} & \text{score} \geq \theta_{\text{upper}} \\ \text{Avoid} & \text{score} \leq \theta_{\text{lower}} \\ \text{Explore} & \text{otherwise} \end{cases}$$

### Memory Model

Agent memory has two tiers:

- **Short-term**: recent observations with strength = 1.0, wiped on decay
- **Long-term**: committed memories with exponential strength decay: $s_i(t) = s_i(0) \cdot \alpha^t$

Memories below `min_strength` are pruned. This implements a **forgetting curve** consistent with Ebbinghaus-type memory models.

### Agent Pool Dynamics

The pool supports fitness-based ranking. At each tick:

$$\bar{f}(t) = \frac{1}{N} \sum_{i=1}^{N} f_i(t)$$

The ranked order enables selection, culling, and population-level health metrics.

### Message Bus

Inter-agent communication uses a simple inbox model: `send(msg)` pushes to a `HashMap<u64, Vec<AgentMessage>>`, and `receive(id)` drains the inbox. Broadcast sends to all recipients except the sender. This is O(1) per send, O(K) per receive where K = pending messages.

### Complexity

| Operation | Time |
|-----------|------|
| `Agent::tick(score)` | O(1) + strategy cost |
| `AgentMemory::observe` | O(1) amortized |
| `AgentMemory::recall(key)` | O(S + L) where S = short-term, L = long-term |
| `AgentMemory::decay` | O(L) |
| `AgentPool::tick_all(score)` | O(N) |
| `AgentPool::ranked()` | O(N log N) |
| `AgentCommunication::send(msg)` | O(1) amortized |

## Quick Start

```rust
use ternary_agent::{Agent, AgentPool, ThresholdStrategy, AgentBehavior, TernaryState};

// Create an agent with threshold behavior
let strategy = ThresholdStrategy::new(-0.5, 0.5);
let behavior = AgentBehavior::new("threshold", Box::new(strategy));
let mut agent = Agent::new(1).with_behavior(behavior);

agent.observe("food", "left");
agent.tick(0.8);  // score > upper threshold → Choose
assert_eq!(agent.state, TernaryState::Choose);

// Pool with ranking
let mut pool = AgentPool::new();
pool.add(Agent::new(1));
pool.add(Agent::new(2));
pool.tick_all(0.5);
let best_first: Vec<u64> = pool.ranked();
```

## API

### Core Types

| Type | Description |
|------|-------------|
| `TernaryState` | Enum: Avoid (-1), Explore (0), Choose (+1) |
| `Agent` | Core agent: id, state, fitness, memory, behavior |
| `AgentMemory` | Short-term + long-term memory with decay |
| `AgentPool` | Collection with fitness ranking |
| `AgentCommunication` | Message bus for inter-agent messaging |
| `Strategy` (trait) | `decide(current_state, score) → TernaryState` |
| `ThresholdStrategy` | Concrete strategy with upper/lower thresholds |
| `AgentBehavior` | Wraps a Strategy with decision counting |

## Architecture Notes

The agent system is built around the **γ + η = C** conservation principle:

- **γ (structure)**: the agent pool topology — who exists, their IDs, their strategy assignments
- **η (dynamics)**: the stream of scores, messages, and observations that perturb agent states
- **C (conservation)**: the invariants — total agent count (no loss), fitness bounds, message delivery guarantees

The ternary state space (|S| = 3) ensures that population-level statistics are always meaningful: with N agents, the macro-state is fully described by $(n_{-1}, n_0, n_{+1})$ where $n_{-1} + n_0 + n_{+1} = N$. The signed charge $\gamma = n_{+1} - n_{-1}$ is a sufficient statistic for population mood.

## References

- Russell, S. & Norvig, P. (2020). *Artificial Intelligence: A Modern Approach* (4th ed.). — Agent taxonomy.
- Wooldridge, M. (2009). *An Introduction to MultiAgent Systems* (2nd ed.). Wiley.
- Brusentsov, N.P. (1958). *Ternary Computers* — Setun system and balanced ternary.
- Ebbinghaus, H. (1885). *Memory: A Contribution to Experimental Psychology*. — Forgetting curves.
- Nowak, M. (2006). *Evolutionary Dynamics*. Harvard — Z₃ cyclic dominance in evolutionary game theory.

## License: MIT
