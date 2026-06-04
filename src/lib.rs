#![forbid(unsafe_code)]

//! Core agent types for the ternary ecosystem.
//!
//! Every agent operates in one of three states — Avoid, Explore, or Choose —
//! borrowed from the ternary digit set {-1, 0, +1}. An `Agent` carries memory,
//! fitness, and behavior. An `AgentPool` tracks a collection of agents with
//! fitness-based ranking. `AgentCommunication` provides message passing.

use std::collections::HashMap;

// ── Ternary State ──────────────────────────────────────────────────────────

/// The three possible states of an agent.
///
/// Maps directly to balanced ternary digits:
/// - `Avoid` = −1 → the agent retreats or rejects
/// - `Explore` = 0 → the agent gathers information
/// - `Choose` = +1 → the agent commits or selects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TernaryState {
    Avoid,
    Explore,
    Choose,
}

impl TernaryState {
    /// Convert to the balanced ternary digit: −1, 0, or +1.
    pub fn to_trit(self) -> i8 {
        match self {
            TernaryState::Avoid => -1,
            TernaryState::Explore => 0,
            TernaryState::Choose => 1,
        }
    }

    /// Convert from a balanced ternary digit.
    ///
    /// Panics if `trit` is not −1, 0, or +1.
    pub fn from_trit(trit: i8) -> Self {
        match trit {
            -1 => TernaryState::Avoid,
            0 => TernaryState::Explore,
            1 => TernaryState::Choose,
            _ => panic!("from_trit: expected -1, 0, or 1, got {}", trit),
        }
    }

    /// Cycle to the next state: Avoid → Explore → Choose → Avoid.
    pub fn next(self) -> Self {
        match self {
            TernaryState::Avoid => TernaryState::Explore,
            TernaryState::Explore => TernaryState::Choose,
            TernaryState::Choose => TernaryState::Avoid,
        }
    }
}

// ── Agent Memory ───────────────────────────────────────────────────────────

/// A single memory entry with an associated strength.
///
/// Strength decays over time when the agent ticks its memory.
#[derive(Debug, Clone)]
pub struct MemoryEntry {
    pub key: String,
    pub value: String,
    pub strength: f64,
}

/// Agent memory split into short-term (recent observations) and long-term
/// (knowledge that persists but decays).
///
/// Short-term entries are cleared when `decay` is called with `clear_short_term = true`.
/// Long-term entries have their strength reduced by `long_term_factor` on each decay.
#[derive(Debug, Clone)]
pub struct AgentMemory {
    pub short_term: Vec<MemoryEntry>,
    pub long_term: Vec<MemoryEntry>,
}

impl AgentMemory {
    pub fn new() -> Self {
        Self {
            short_term: Vec::new(),
            long_term: Vec::new(),
        }
    }

    /// Add an observation to short-term memory.
    pub fn observe(&mut self, key: &str, value: &str) {
        self.short_term.push(MemoryEntry {
            key: key.to_string(),
            value: value.to_string(),
            strength: 1.0,
        });
    }

    /// Commit a short-term memory into long-term memory with initial strength 1.0.
    ///
    /// Returns false if no short-term entry with that key exists.
    pub fn commit(&mut self, key: &str) -> bool {
        if let Some(pos) = self.short_term.iter().position(|e| e.key == key) {
            let entry = self.short_term.remove(pos);
            self.long_term.push(MemoryEntry {
                strength: 1.0,
                ..entry
            });
            true
        } else {
            false
        }
    }

    /// Recall a memory — checks short-term first, then long-term.
    /// Returns the value and current strength, or None if not found.
    pub fn recall(&self, key: &str) -> Option<(&str, f64)> {
        if let Some(entry) = self.short_term.iter().find(|e| e.key == key) {
            return Some((&entry.value, entry.strength));
        }
        self.long_term
            .iter()
            .find(|e| e.key == key)
            .map(|e| (&*e.value, e.strength))
    }

    /// Decay memories. If `clear_short_term` is true, short-term is wiped.
    /// Long-term entries have their strength multiplied by `long_term_factor`.
    /// Entries below `min_strength` are pruned.
    pub fn decay(&mut self, clear_short_term: bool, long_term_factor: f64, min_strength: f64) {
        if clear_short_term {
            self.short_term.clear();
        }
        for entry in &mut self.long_term {
            entry.strength *= long_term_factor;
        }
        self.long_term
            .retain(|e| e.strength >= min_strength);
    }

    /// Total number of memories (short + long).
    pub fn len(&self) -> usize {
        self.short_term.len() + self.long_term.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for AgentMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ── Agent Behavior ─────────────────────────────────────────────────────────

/// A strategy returns a ternary state given the agent's current state and
/// a numeric score (fitness or observation).
pub trait Strategy: std::fmt::Debug {
    fn decide(&self, current: TernaryState, score: f64) -> TernaryState;
}

/// Threshold-based strategy: switch to Choose if score ≥ upper threshold,
/// switch to Avoid if score ≤ lower threshold, otherwise Explore.
#[derive(Debug, Clone)]
pub struct ThresholdStrategy {
    pub lower: f64,
    pub upper: f64,
}

impl ThresholdStrategy {
    pub fn new(lower: f64, upper: f64) -> Self {
        Self { lower, upper }
    }
}

impl Strategy for ThresholdStrategy {
    fn decide(&self, _current: TernaryState, score: f64) -> TernaryState {
        if score >= self.upper {
            TernaryState::Choose
        } else if score <= self.lower {
            TernaryState::Avoid
        } else {
            TernaryState::Explore
        }
    }
}

/// A behavior wraps a strategy and tracks how many decisions have been made.
pub struct AgentBehavior {
    pub strategy_name: String,
    pub decisions_made: u64,
    strategy: Box<dyn Strategy>,
}

impl std::fmt::Debug for AgentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentBehavior")
            .field("strategy_name", &self.strategy_name)
            .field("decisions_made", &self.decisions_made)
            .finish()
    }
}

impl AgentBehavior {
    pub fn new(name: &str, strategy: Box<dyn Strategy>) -> Self {
        Self {
            strategy_name: name.to_string(),
            decisions_made: 0,
            strategy,
        }
    }

    /// Execute the strategy and increment the decision counter.
    pub fn execute(&mut self, current: TernaryState, score: f64) -> TernaryState {
        let result = self.strategy.decide(current, score);
        self.decisions_made += 1;
        result
    }
}

// ── Agent Communication ───────────────────────────────────────────────────

/// A message between agents.
#[derive(Debug, Clone)]
pub struct AgentMessage {
    pub from: u64,
    pub to: u64,
    pub tag: String,
    pub payload: String,
}

/// Message bus for inter-agent communication.
#[derive(Debug, Clone)]
pub struct AgentCommunication {
    inbox: HashMap<u64, Vec<AgentMessage>>,
}

impl AgentCommunication {
    pub fn new() -> Self {
        Self {
            inbox: HashMap::new(),
        }
    }

    /// Send a message. The message is pushed to the recipient's inbox.
    pub fn send(&mut self, msg: AgentMessage) {
        self.inbox.entry(msg.to).or_default().push(msg);
    }

    /// Receive all pending messages for `agent_id`, removing them from the inbox.
    pub fn receive(&mut self, agent_id: u64) -> Vec<AgentMessage> {
        self.inbox.remove(&agent_id).unwrap_or_default()
    }

    /// Peek at pending message count for an agent without consuming them.
    pub fn pending(&self, agent_id: u64) -> usize {
        self.inbox.get(&agent_id).map(|v| v.len()).unwrap_or(0)
    }

    /// Broadcast a message from `from_id` to all known recipients.
    pub fn broadcast(&mut self, from_id: u64, tag: &str, payload: &str, recipients: &[u64]) {
        for &to in recipients {
            if to != from_id {
                self.send(AgentMessage {
                    from: from_id,
                    to,
                    tag: tag.to_string(),
                    payload: payload.to_string(),
                });
            }
        }
    }
}

impl Default for AgentCommunication {
    fn default() -> Self {
        Self::new()
    }
}

// ── Agent ──────────────────────────────────────────────────────────────────

/// The core agent type.
///
/// Each agent has an ID, a ternary state, a fitness score, memory, and an
/// optional behavior. The agent does not run autonomously — it is driven by
/// external calls to `tick`.
#[derive(Debug)]
pub struct Agent {
    pub id: u64,
    pub state: TernaryState,
    pub fitness: f64,
    pub memory: AgentMemory,
    pub behavior: Option<AgentBehavior>,
}

impl Agent {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            state: TernaryState::Explore,
            fitness: 0.0,
            memory: AgentMemory::new(),
            behavior: None,
        }
    }

    /// Attach a behavior strategy.
    pub fn with_behavior(mut self, behavior: AgentBehavior) -> Self {
        self.behavior = Some(behavior);
        self
    }

    /// Tick the agent: run the behavior strategy (if any) to update state,
    /// then decay memory.
    pub fn tick(&mut self, score: f64) {
        if let Some(ref mut b) = self.behavior {
            self.state = b.execute(self.state, score);
        }
        self.fitness = score;
    }

    /// Observe something into short-term memory.
    pub fn observe(&mut self, key: &str, value: &str) {
        self.memory.observe(key, value);
    }

    /// Commit a short-term memory to long-term.
    pub fn commit_memory(&mut self, key: &str) -> bool {
        self.memory.commit(key)
    }

    /// Recall a memory.
    pub fn recall(&self, key: &str) -> Option<(&str, f64)> {
        self.memory.recall(key)
    }
}

// ── Agent Pool ─────────────────────────────────────────────────────────────

/// A collection of agents with fitness tracking and ranking.
#[derive(Debug)]
pub struct AgentPool {
    agents: HashMap<u64, Agent>,
}

impl AgentPool {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Add an agent to the pool.
    pub fn add(&mut self, agent: Agent) {
        self.agents.insert(agent.id, agent);
    }

    /// Remove an agent, returning it if it existed.
    pub fn remove(&mut self, id: u64) -> Option<Agent> {
        self.agents.remove(&id)
    }

    /// Get a reference to an agent.
    pub fn get(&self, id: u64) -> Option<&Agent> {
        self.agents.get(&id)
    }

    /// Get a mutable reference to an agent.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut Agent> {
        self.agents.get_mut(&id)
    }

    /// Number of agents in the pool.
    pub fn len(&self) -> usize {
        self.agents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// Return agent IDs sorted by fitness descending (best first).
    pub fn ranked(&self) -> Vec<u64> {
        let mut ids: Vec<&Agent> = self.agents.values().collect();
        ids.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        ids.iter().map(|a| a.id).collect()
    }

    /// Average fitness across all agents. Returns 0.0 for empty pool.
    pub fn avg_fitness(&self) -> f64 {
        if self.agents.is_empty() {
            return 0.0;
        }
        self.agents.values().map(|a| a.fitness).sum::<f64>() / self.agents.len() as f64
    }

    /// Tick all agents with the same score.
    pub fn tick_all(&mut self, score: f64) {
        for agent in self.agents.values_mut() {
            agent.tick(score);
        }
    }

    /// Return all agent IDs.
    pub fn ids(&self) -> Vec<u64> {
        self.agents.keys().copied().collect()
    }
}

impl Default for AgentPool {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ternary_state_trit_roundtrip() {
        assert_eq!(TernaryState::Avoid.to_trit(), -1);
        assert_eq!(TernaryState::Explore.to_trit(), 0);
        assert_eq!(TernaryState::Choose.to_trit(), 1);
        assert_eq!(TernaryState::from_trit(-1), TernaryState::Avoid);
        assert_eq!(TernaryState::from_trit(0), TernaryState::Explore);
        assert_eq!(TernaryState::from_trit(1), TernaryState::Choose);
    }

    #[test]
    #[should_panic]
    fn ternary_state_invalid_trit() {
        TernaryState::from_trit(2);
    }

    #[test]
    fn ternary_state_cycle() {
        let mut s = TernaryState::Avoid;
        s = s.next();
        assert_eq!(s, TernaryState::Explore);
        s = s.next();
        assert_eq!(s, TernaryState::Choose);
        s = s.next();
        assert_eq!(s, TernaryState::Avoid);
    }

    #[test]
    fn agent_new_defaults() {
        let a = Agent::new(42);
        assert_eq!(a.id, 42);
        assert_eq!(a.state, TernaryState::Explore);
        assert_eq!(a.fitness, 0.0);
        assert!(a.memory.is_empty());
        assert!(a.behavior.is_none());
    }

    #[test]
    fn agent_tick_without_behavior() {
        let mut a = Agent::new(1);
        a.tick(0.5);
        assert_eq!(a.fitness, 0.5);
        assert_eq!(a.state, TernaryState::Explore); // no behavior → state unchanged
    }

    #[test]
    fn agent_tick_with_behavior() {
        let strat = ThresholdStrategy::new(-0.5, 0.5);
        let behavior = AgentBehavior::new("threshold", Box::new(strat));
        let mut a = Agent::new(1).with_behavior(behavior);
        a.tick(0.8);
        assert_eq!(a.state, TernaryState::Choose);
        assert_eq!(a.fitness, 0.8);
    }

    #[test]
    fn agent_memory_observe_and_recall() {
        let mut a = Agent::new(1);
        a.observe("food", "left");
        let (val, strength) = a.recall("food").unwrap();
        assert_eq!(val, "left");
        assert_eq!(strength, 1.0);
    }

    #[test]
    fn agent_memory_commit() {
        let mut a = Agent::new(1);
        a.observe("food", "left");
        assert!(a.commit_memory("food"));
        assert!(a.recall("food").is_some()); // now in long-term
        assert!(!a.commit_memory("nonexistent"));
    }

    #[test]
    fn memory_decay_prunes_weak() {
        let mut mem = AgentMemory::new();
        mem.observe("temp", "data");
        mem.commit("temp");
        // long-term strength is now 1.0; decay to 0.05
        mem.decay(false, 0.05, 0.1);
        assert!(mem.long_term.is_empty()); // pruned below 0.1
    }

    #[test]
    fn memory_decay_clears_short_term() {
        let mut mem = AgentMemory::new();
        mem.observe("x", "y");
        mem.decay(true, 0.9, 0.0);
        assert!(mem.short_term.is_empty());
        assert!(mem.long_term.is_empty());
    }

    #[test]
    fn agent_pool_add_remove() {
        let mut pool = AgentPool::new();
        pool.add(Agent::new(1));
        pool.add(Agent::new(2));
        assert_eq!(pool.len(), 2);
        let removed = pool.remove(1);
        assert!(removed.is_some());
        assert_eq!(pool.len(), 1);
    }

    #[test]
    fn agent_pool_ranked() {
        let mut pool = AgentPool::new();
        let mut a1 = Agent::new(1);
        a1.fitness = 0.9;
        let mut a2 = Agent::new(2);
        a2.fitness = 0.3;
        let mut a3 = Agent::new(3);
        a3.fitness = 0.6;
        pool.add(a1);
        pool.add(a2);
        pool.add(a3);
        assert_eq!(pool.ranked(), vec![1, 3, 2]);
    }

    #[test]
    fn agent_pool_avg_fitness() {
        let mut pool = AgentPool::new();
        assert_eq!(pool.avg_fitness(), 0.0);
        let mut a1 = Agent::new(1);
        a1.fitness = 1.0;
        let mut a2 = Agent::new(2);
        a2.fitness = 0.5;
        pool.add(a1);
        pool.add(a2);
        assert_eq!(pool.avg_fitness(), 0.75);
    }

    #[test]
    fn agent_pool_tick_all() {
        let strat = ThresholdStrategy::new(-0.5, 0.5);
        let behavior = AgentBehavior::new("t", Box::new(strat));
        let mut pool = AgentPool::new();
        pool.add(Agent::new(1));
        pool.add(Agent::new(2).with_behavior(behavior));
        pool.tick_all(0.8);
        assert_eq!(pool.get(1).unwrap().fitness, 0.8);
        assert_eq!(pool.get(2).unwrap().state, TernaryState::Choose);
    }

    #[test]
    fn communication_send_receive() {
        let mut bus = AgentCommunication::new();
        bus.send(AgentMessage { from: 1, to: 2, tag: "hello".into(), payload: "world".into() });
        assert_eq!(bus.pending(2), 1);
        let msgs = bus.receive(2);
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].tag, "hello");
        assert_eq!(bus.pending(2), 0);
    }

    #[test]
    fn communication_broadcast() {
        let mut bus = AgentCommunication::new();
        bus.broadcast(1, "alert", "fire", &[1, 2, 3]);
        assert_eq!(bus.pending(1), 0); // sender excluded
        assert_eq!(bus.pending(2), 1);
        assert_eq!(bus.pending(3), 1);
    }

    #[test]
    fn behavior_decision_count() {
        let strat = ThresholdStrategy::new(0.0, 1.0);
        let mut b = AgentBehavior::new("test", Box::new(strat));
        b.execute(TernaryState::Explore, 0.5);
        b.execute(TernaryState::Explore, 1.5);
        assert_eq!(b.decisions_made, 2);
    }

    #[test]
    fn agent_pool_get_mut() {
        let mut pool = AgentPool::new();
        pool.add(Agent::new(10));
        pool.get_mut(10).unwrap().fitness = 99.0;
        assert_eq!(pool.get(10).unwrap().fitness, 99.0);
    }

    #[test]
    fn agent_pool_ids() {
        let mut pool = AgentPool::new();
        pool.add(Agent::new(5));
        pool.add(Agent::new(10));
        let mut ids = pool.ids();
        ids.sort();
        assert_eq!(ids, vec![5, 10]);
    }

    #[test]
    fn recall_missing_key() {
        let mem = AgentMemory::new();
        assert!(mem.recall("nope").is_none());
    }

    #[test]
    fn pool_default_is_empty() {
        let pool = AgentPool::default();
        assert!(pool.is_empty());
    }

    #[test]
    fn communication_default() {
        let bus = AgentCommunication::default();
        assert_eq!(bus.pending(0), 0);
    }

    #[test]
    fn memory_len_counts_both() {
        let mut mem = AgentMemory::new();
        mem.observe("a", "1");
        mem.observe("b", "2");
        mem.commit("a");
        assert_eq!(mem.len(), 2); // 1 short + 1 long
    }
}
