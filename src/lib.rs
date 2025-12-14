//! # Warhorn
//!
//! Protocol types for AI agent communication - signals between goblins.
//!
//! This crate defines the message protocol inspired by codex-kaioken's Op/Event pattern:
//! - **Operations (Op)**: Messages sent FROM UI TO agent orchestrator
//! - **Events**: Messages sent FROM agent orchestrator TO UI
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────┐     Op      ┌──────────────────┐
//! │     UI       │ ──────────▶ │      Cabal       │
//! │  (Terminal)  │             │  (Orchestrator)  │
//! │              │ ◀────────── │                  │
//! └──────────────┘    Event    └──────────────────┘
//! ```
//!
//! ## Transport Agnostic
//!
//! These types can be serialized over any transport:
//! - In-process channels (tokio mpsc)
//! - Unix domain sockets
//! - stdio (for MCP compatibility)
//! - WebSocket (for remote agents)

pub mod ids;
pub mod ops;
pub mod events;
pub mod models;
pub mod error;

pub use ids::*;
pub use ops::Op;
pub use events::Event;
pub use models::*;
pub use error::ProtocolError;

/// Protocol version for compatibility checking
pub const PROTOCOL_VERSION: &str = "0.1.0";
