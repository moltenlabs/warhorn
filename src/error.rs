//! Protocol error types

use thiserror::Error;

/// Errors that can occur in protocol handling
#[derive(Debug, Error)]
pub enum ProtocolError {
    /// Failed to serialize message
    #[error("Failed to serialize message: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Failed to deserialize message
    #[error("Failed to deserialize message: {message}")]
    DeserializationError { message: String },

    /// Unknown operation type
    #[error("Unknown operation type: {0}")]
    UnknownOperation(String),

    /// Unknown event type
    #[error("Unknown event type: {0}")]
    UnknownEvent(String),

    /// Invalid submission ID
    #[error("Invalid submission ID: {0}")]
    InvalidSubmissionId(String),

    /// Protocol version mismatch
    #[error("Protocol version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: String, actual: String },

    /// Transport error
    #[error("Transport error: {0}")]
    TransportError(String),

    /// Channel closed
    #[error("Channel closed")]
    ChannelClosed,
}
