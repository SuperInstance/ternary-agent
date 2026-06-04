# Future Integration: ternary-agent

## Current State

ternary-agent defines the core agent types for the ternary ecosystem. `Agent` carries an ID, `TernaryState` (Avoid/Explore/Choose mapping to -1/0/+1), fitness, `AgentMemory` (short-term + long-term with decay), and optional `AgentBehavior` wrapping a `Strategy` trait. `ThresholdStrategy` switches states based on score thresholds. `AgentPool` manages collections with fitness-based ranking (`ranked()` returns IDs sorted by fitness descending). `AgentCommunication` provides a message bus with `send()`, `receive()`, `broadcast()`, and `pending()` for inter-agent messaging.

## Integration Opportunities

### Capitaine-1 Vessel Classes

The `Agent` struct maps directly to capitaine-1 vessel classes. An Agent's `TernaryState::Choose` = Capitaine (flagship, committing to action), `TernaryState::Explore` = Éclaireur (scout, gathering information), `TernaryState::Avoid` = Sentinelle (sentinel, retreating from threats). The `AgentBehavior` with its `Strategy` trait determines vessel role: a `ThresholdStrategy` with tight thresholds = Marksman, wide thresholds = Explorer. Adding a `vessel_class` field to `Agent` and a `VesselStrategy` trait (extending `Strategy` with vessel-specific decisions) formalizes this mapping.

### ternary-cell → Agent as Cell Colony

An `Agent` in a PLATO room IS a `TernaryCell` colony. The agent's `tick(score)` maps to the cell's six-phase tick. `AgentMemory` provides the prediction (short-term observations → predict next state), `AgentBehavior::execute()` is the perceive+surprise phase, and fitness tracking is the conservation check. Integration: `Agent` wraps a `CellGrid` where internal cells represent the agent's sub-goals or knowledge fragments.

### ternary-ensign → Specialist Behavior

`AgentBehavior` currently wraps a single `Strategy`. The ensign pattern enables multi-strategy agents: load different strategies as ensigns when entering rooms. An `AgentPool` becomes a fleet where each agent has different specialists loaded. `AgentCommunication` becomes the inter-ensign messaging layer — when the "engine-monitor" ensign detects an anomaly (via `AgentMessage` with tag "anomaly"), it broadcasts to the fleet.

### I2I Protocol via AgentCommunication

`AgentCommunication`'s `send()`/`receive()` map to I2I bottle protocol: `AgentMessage { from, to, tag, payload }` IS an I2I TELL message. Tag = I2I message type (TELL, ASK, ALERT). Adding `broadcast_from_room()` that only sends to agents in the same room enables room-scoped coordination. The `pending()` count becomes a fleet health metric — agents with growing inboxes are overloaded.

## Potential in Mature Systems

In a mature fleet, every git-agent (vessel) IS an `Agent` with a `TernaryState`, fitness score, memory, and behavior. The `AgentPool` tracks the entire fleet. `ranked()` determines which agents get priority tasks. `avg_fitness()` is the fleet health score. When `avg_fitness()` drops below a threshold, the fleet coordinator (Oracle1) spawns new agents or retires underperformers. The three-state lifecycle (Avoid = retire, Explore = scout new tasks, Choose = commit to a task) drives autonomous fleet management.

## Cross-Pollination Ideas

- **Agent memory → PLATO tiles**: `AgentMemory::commit()` extracts short-term observations into long-term memory. In PLATO, this maps to generating tiles from room experiences and syncing to the tile store.
- **Fitness ranking → natural selection**: `AgentPool::ranked()` + `ternary-evolution`'s genetic operators = evolve agents. Bottom-ranked agents are replaced with crossover/mutation of top-ranked agents.
- **AgentCommunication → ternary-protocol bridge**: `AgentMessage` serializes into `ternary-protocol::TernaryMessage` for wire transport. Tag → message type, payload → `Payload`.

## Dependencies for Next Steps

1. `VesselStrategy` trait extending `Strategy` with vessel-class awareness
2. `Agent` → `CellGrid` bridge for room-as-codespace integration
3. `AgentCommunication` → `ternary-protocol` serialization layer
4. Fleet coordinator that uses `AgentPool::ranked()` for task assignment
