# 📯 Warhorn

Protocol types for AI agent communication - signals between goblins.

[![Crates.io](https://img.shields.io/crates/v/warhorn.svg)](https://crates.io/crates/warhorn)
[![Documentation](https://docs.rs/warhorn/badge.svg)](https://docs.rs/warhorn)
[![License](https://img.shields.io/crates/l/warhorn.svg)](LICENSE)

## Overview

Warhorn defines the message protocol for communication between AI agents and their orchestrators. Inspired by codex-kaioken's Op/Event pattern:

- **Operations (Op)**: Commands sent FROM UI TO agent orchestrator
- **Events**: Notifications sent FROM orchestrator TO UI

## Features

- 🆔 Strongly-typed IDs (`AgentId`, `TaskId`, `SessionId`, etc.)
- 📨 Serializable message types for agent communication
- 🔄 Transport-agnostic (works over channels, sockets, stdio)
- 📊 Rich models for sessions, agents, tasks, and tools

## Installation

```toml
[dependencies]
warhorn = "0.1"
```

## Usage

```rust
use warhorn::{Op, Event, AgentId, SubmissionId};

// Send a user input operation
let op = Op::user_input("Add authentication to my app");

// Handle events from the orchestrator
match event {
    Event::AgentSpawned { agent_id, role, .. } => {
        println!("Agent {} spawned as {:?}", agent_id, role);
    }
    Event::TaskComplete { task_id, result, .. } => {
        println!("Task {} completed: {}", task_id, result.summary);
    }
    _ => {}
}
```

## Part of the Goblin Family

- **warhorn** - Protocol types (you are here)
- [trinkets](https://crates.io/crates/trinkets) - Tool registry
- [wardstone](https://crates.io/crates/wardstone) - Sandboxing
- [skulk](https://crates.io/crates/skulk) - MCP connections
- [hutch](https://crates.io/crates/hutch) - Checkpoints
- [ambush](https://crates.io/crates/ambush) - Task planning
- [cabal](https://crates.io/crates/cabal) - Orchestration

## License

MIT OR Apache-2.0
