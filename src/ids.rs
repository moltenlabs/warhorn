//! Strongly-typed identifiers for protocol entities

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique identifier for an agent in the hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AgentId(Uuid);

impl AgentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for AgentId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "agent-{}", &self.0.to_string()[..8])
    }
}

/// Unique identifier for a task
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "task-{}", &self.0.to_string()[..8])
    }
}

/// Unique identifier for a tool call
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CallId(Uuid);

impl CallId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for CallId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CallId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "call-{}", &self.0.to_string()[..8])
    }
}

/// Unique identifier for a session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "session-{}", &self.0.to_string()[..8])
    }
}

/// Unique identifier for a checkpoint
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CheckpointId(Uuid);

impl CheckpointId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl Default for CheckpointId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CheckpointId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "checkpoint-{}", &self.0.to_string()[..8])
    }
}

/// Submission ID for correlating operations with events
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubmissionId(String);

impl SubmissionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for SubmissionId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SubmissionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === AgentId Tests ===
    
    #[test]
    fn test_agent_id_display() {
        let id = AgentId::new();
        let display = format!("{}", id);
        assert!(display.starts_with("agent-"));
        assert_eq!(display.len(), 14); // "agent-" + 8 chars
    }

    #[test]
    fn test_agent_id_unique() {
        let ids: Vec<AgentId> = (0..100).map(|_| AgentId::new()).collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 100, "All 100 AgentIds should be unique");
    }

    #[test]
    fn test_agent_id_serialization_roundtrip() {
        let id = AgentId::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: AgentId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_agent_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = AgentId::from_uuid(uuid);
        assert_eq!(id.as_uuid(), &uuid);
    }

    #[test]
    fn test_agent_id_default() {
        let id1: AgentId = Default::default();
        let id2: AgentId = Default::default();
        assert_ne!(id1, id2, "Default should create new IDs");
    }

    #[test]
    fn test_agent_id_hash() {
        use std::collections::HashMap;
        let id = AgentId::new();
        let mut map = HashMap::new();
        map.insert(id, "test");
        assert_eq!(map.get(&id), Some(&"test"));
    }

    #[test]
    fn test_agent_id_clone() {
        let id = AgentId::new();
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    // === TaskId Tests ===

    #[test]
    fn test_task_id_display() {
        let id = TaskId::new();
        let display = format!("{}", id);
        assert!(display.starts_with("task-"));
        assert_eq!(display.len(), 13); // "task-" + 8 chars
    }

    #[test]
    fn test_task_id_unique() {
        let ids: Vec<TaskId> = (0..100).map(|_| TaskId::new()).collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 100);
    }

    #[test]
    fn test_task_id_serialization() {
        let id = TaskId::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: TaskId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_task_id_from_uuid() {
        let uuid = Uuid::new_v4();
        let id = TaskId::from_uuid(uuid);
        let display = format!("{}", id);
        assert!(display.starts_with("task-"));
    }

    // === CallId Tests ===

    #[test]
    fn test_call_id_display() {
        let id = CallId::new();
        let display = format!("{}", id);
        assert!(display.starts_with("call-"));
        assert_eq!(display.len(), 13); // "call-" + 8 chars
    }

    #[test]
    fn test_call_id_unique() {
        let ids: Vec<CallId> = (0..100).map(|_| CallId::new()).collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 100);
    }

    #[test]
    fn test_call_id_serialization() {
        let id = CallId::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: CallId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    // === SessionId Tests ===

    #[test]
    fn test_session_id_display() {
        let id = SessionId::new();
        let display = format!("{}", id);
        assert!(display.starts_with("session-"));
        assert_eq!(display.len(), 16); // "session-" + 8 chars
    }

    #[test]
    fn test_session_id_unique() {
        let ids: Vec<SessionId> = (0..100).map(|_| SessionId::new()).collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 100);
    }

    #[test]
    fn test_session_id_serialization() {
        let id = SessionId::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: SessionId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    // === CheckpointId Tests ===

    #[test]
    fn test_checkpoint_id_display() {
        let id = CheckpointId::new();
        let display = format!("{}", id);
        assert!(display.starts_with("checkpoint-"));
        assert_eq!(display.len(), 19); // "checkpoint-" + 8 chars
    }

    #[test]
    fn test_checkpoint_id_unique() {
        let ids: Vec<CheckpointId> = (0..100).map(|_| CheckpointId::new()).collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 100);
    }

    #[test]
    fn test_checkpoint_id_serialization() {
        let id = CheckpointId::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: CheckpointId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    // === SubmissionId Tests ===

    #[test]
    fn test_submission_id_display() {
        let id = SubmissionId::new();
        let display = format!("{}", id);
        assert_eq!(display.len(), 36); // UUID string length
    }

    #[test]
    fn test_submission_id_from_string() {
        let custom = SubmissionId::from_string("custom-id");
        assert_eq!(custom.as_str(), "custom-id");
    }

    #[test]
    fn test_submission_id_unique() {
        let ids: Vec<SubmissionId> = (0..100).map(|_| SubmissionId::new()).collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 100);
    }

    #[test]
    fn test_submission_id_serialization() {
        let id = SubmissionId::new();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: SubmissionId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_submission_id_default() {
        let id: SubmissionId = Default::default();
        assert!(!id.as_str().is_empty());
    }
}
