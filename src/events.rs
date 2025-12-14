//! Events sent from Goblin orchestrator to UI
//!
//! These are notifications about agent activity, state changes, and results.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::ids::*;
use crate::models::*;

/// Events sent FROM Goblin orchestrator TO Lair UI
///
/// Each event includes a `sub_id` correlating to the operation that triggered it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Event {
    // === Session Events ===

    /// Session has been configured/reconfigured
    SessionConfigured {
        sub_id: SubmissionId,
        session_id: SessionId,
        config: SessionConfig,
    },

    /// Session settings updated
    SettingsUpdated {
        sub_id: SubmissionId,
        settings: SessionSettings,
    },

    // === Task Events ===

    /// A new task has started
    TaskStarted {
        sub_id: SubmissionId,
        task_id: TaskId,
        prompt: String,
    },

    /// A task turn completed (checkpoint for resumption)
    TurnComplete {
        sub_id: SubmissionId,
        task_id: TaskId,
        turn_number: u32,
        /// Checkpoint ID for this turn
        checkpoint_id: CheckpointId,
    },

    /// Task completed successfully
    TaskComplete {
        sub_id: SubmissionId,
        task_id: TaskId,
        result: TaskResult,
    },

    /// Task failed with error
    TaskFailed {
        sub_id: SubmissionId,
        task_id: TaskId,
        error: String,
    },

    /// Task was interrupted
    TaskInterrupted {
        sub_id: SubmissionId,
        task_id: TaskId,
    },

    // === Agent Events ===

    /// New agent spawned in hierarchy
    AgentSpawned {
        sub_id: SubmissionId,
        agent_id: AgentId,
        parent_id: Option<AgentId>,
        role: AgentRole,
        config: AgentConfig,
    },

    /// Agent started working on task
    AgentWorking {
        sub_id: SubmissionId,
        agent_id: AgentId,
        task_summary: String,
    },

    /// Agent status changed
    AgentStatusChanged {
        sub_id: SubmissionId,
        agent_id: AgentId,
        status: AgentStatus,
    },

    /// Streaming message from agent
    AgentMessage {
        sub_id: SubmissionId,
        agent_id: AgentId,
        content: String,
        /// True if more content coming
        streaming: bool,
        /// Message type
        message_type: MessageType,
    },

    /// Agent completed its task
    AgentComplete {
        sub_id: SubmissionId,
        agent_id: AgentId,
        result: AgentResult,
    },

    /// Agent terminated
    AgentTerminated {
        sub_id: SubmissionId,
        agent_id: AgentId,
        reason: String,
    },

    // === Tool Events ===

    /// Tool call started
    ToolCallStart {
        sub_id: SubmissionId,
        agent_id: AgentId,
        call_id: CallId,
        tool_name: String,
        arguments: serde_json::Value,
    },

    /// Tool execution requires approval
    ApprovalRequired {
        sub_id: SubmissionId,
        agent_id: AgentId,
        call_id: CallId,
        tool_name: String,
        arguments: serde_json::Value,
        /// Human-readable description of what will happen
        description: String,
        /// Risk level
        risk: RiskLevel,
    },

    /// Tool call completed
    ToolCallComplete {
        sub_id: SubmissionId,
        agent_id: AgentId,
        call_id: CallId,
        tool_name: String,
        output: ToolOutput,
        duration_ms: u64,
    },

    /// Tool call failed
    ToolCallFailed {
        sub_id: SubmissionId,
        agent_id: AgentId,
        call_id: CallId,
        tool_name: String,
        error: String,
    },

    // === Hierarchy Events ===

    /// Agent hierarchy changed
    HierarchyUpdated {
        sub_id: SubmissionId,
        root: AgentTree,
    },

    // === Checkpoint Events ===

    /// Checkpoint saved
    CheckpointSaved {
        sub_id: SubmissionId,
        checkpoint_id: CheckpointId,
        name: Option<String>,
        timestamp: DateTime<Utc>,
    },

    /// Checkpoint restored
    CheckpointRestored {
        sub_id: SubmissionId,
        checkpoint_id: CheckpointId,
    },

    /// List of checkpoints
    CheckpointList {
        sub_id: SubmissionId,
        checkpoints: Vec<CheckpointMeta>,
    },

    // === Plan Events ===

    /// Plan mode toggled
    PlanModeChanged {
        sub_id: SubmissionId,
        enabled: bool,
        granularity: PlanGranularity,
    },

    /// Plan created from user request
    PlanCreated {
        sub_id: SubmissionId,
        plan: TaskPlan,
    },

    // === System Events ===

    /// Non-fatal warning
    Warning {
        sub_id: SubmissionId,
        message: String,
        #[serde(default)]
        details: Option<String>,
    },

    /// Fatal error
    Error {
        sub_id: SubmissionId,
        message: String,
        #[serde(default)]
        recoverable: bool,
    },

    /// Token/cost usage update
    UsageUpdate {
        sub_id: SubmissionId,
        agent_id: Option<AgentId>,
        usage: TokenUsage,
    },
}

impl Event {
    /// Get the submission ID for this event
    pub fn sub_id(&self) -> &SubmissionId {
        match self {
            Event::SessionConfigured { sub_id, .. } => sub_id,
            Event::SettingsUpdated { sub_id, .. } => sub_id,
            Event::TaskStarted { sub_id, .. } => sub_id,
            Event::TurnComplete { sub_id, .. } => sub_id,
            Event::TaskComplete { sub_id, .. } => sub_id,
            Event::TaskFailed { sub_id, .. } => sub_id,
            Event::TaskInterrupted { sub_id, .. } => sub_id,
            Event::AgentSpawned { sub_id, .. } => sub_id,
            Event::AgentWorking { sub_id, .. } => sub_id,
            Event::AgentStatusChanged { sub_id, .. } => sub_id,
            Event::AgentMessage { sub_id, .. } => sub_id,
            Event::AgentComplete { sub_id, .. } => sub_id,
            Event::AgentTerminated { sub_id, .. } => sub_id,
            Event::ToolCallStart { sub_id, .. } => sub_id,
            Event::ApprovalRequired { sub_id, .. } => sub_id,
            Event::ToolCallComplete { sub_id, .. } => sub_id,
            Event::ToolCallFailed { sub_id, .. } => sub_id,
            Event::HierarchyUpdated { sub_id, .. } => sub_id,
            Event::CheckpointSaved { sub_id, .. } => sub_id,
            Event::CheckpointRestored { sub_id, .. } => sub_id,
            Event::CheckpointList { sub_id, .. } => sub_id,
            Event::PlanModeChanged { sub_id, .. } => sub_id,
            Event::PlanCreated { sub_id, .. } => sub_id,
            Event::Warning { sub_id, .. } => sub_id,
            Event::Error { sub_id, .. } => sub_id,
            Event::UsageUpdate { sub_id, .. } => sub_id,
        }
    }

    /// Check if this is an error event
    pub fn is_error(&self) -> bool {
        matches!(self, Event::Error { .. } | Event::TaskFailed { .. })
    }

    /// Check if this event requires UI attention
    pub fn requires_attention(&self) -> bool {
        matches!(
            self,
            Event::ApprovalRequired { .. } | Event::Error { .. } | Event::Warning { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Task Event Tests ===

    #[test]
    fn test_task_started_event() {
        let event = Event::TaskStarted {
            sub_id: SubmissionId::new(),
            task_id: TaskId::new(),
            prompt: "Test prompt".to_string(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("task_started"));
        assert!(json.contains("Test prompt"));

        let parsed: Event = serde_json::from_str(&json).unwrap();
        assert!(!parsed.is_error());
        assert!(!parsed.requires_attention());
    }

    #[test]
    fn test_task_complete_event() {
        let event = Event::TaskComplete {
            sub_id: SubmissionId::new(),
            task_id: TaskId::new(),
            result: TaskResult {
                task_id: TaskId::new(),
                success: true,
                summary: "Done!".into(),
                files_changed: vec![],
                token_usage: TokenUsage::default(),
            },
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("task_complete"));
        assert!(json.contains("Done!"));
        assert!(!event.is_error());
    }

    #[test]
    fn test_task_failed_event() {
        let event = Event::TaskFailed {
            sub_id: SubmissionId::new(),
            task_id: TaskId::new(),
            error: "Something failed".into(),
        };
        
        assert!(event.is_error());
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("task_failed"));
    }

    #[test]
    fn test_task_interrupted_event() {
        let event = Event::TaskInterrupted {
            sub_id: SubmissionId::new(),
            task_id: TaskId::new(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("task_interrupted"));
        assert!(!event.is_error());
    }

    #[test]
    fn test_turn_complete_event() {
        let event = Event::TurnComplete {
            sub_id: SubmissionId::new(),
            task_id: TaskId::new(),
            turn_number: 5,
            checkpoint_id: CheckpointId::new(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("turn_complete"));
        assert!(json.contains("turn_number"));
    }

    // === Agent Event Tests ===

    #[test]
    fn test_agent_spawned_event() {
        let event = Event::AgentSpawned {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            parent_id: None,
            role: AgentRole::Worker,
            config: AgentConfig::default(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("agent_spawned"));
        assert!(json.contains("worker"));
    }

    #[test]
    fn test_agent_spawned_with_parent() {
        let parent_id = AgentId::new();
        let event = Event::AgentSpawned {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            parent_id: Some(parent_id),
            role: AgentRole::DomainLead { domain: "frontend".into() },
            config: AgentConfig::default(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("agent_spawned"));
        assert!(json.contains("frontend"));
    }

    #[test]
    fn test_agent_working_event() {
        let event = Event::AgentWorking {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            task_summary: "Implementing feature X".into(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("agent_working"));
        assert!(json.contains("Implementing feature X"));
    }

    #[test]
    fn test_agent_status_changed_event() {
        let event = Event::AgentStatusChanged {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            status: AgentStatus::Running,
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("agent_status_changed"));
        assert!(json.contains("running"));
    }

    #[test]
    fn test_agent_message_event() {
        let event = Event::AgentMessage {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            content: "Working on it...".into(),
            streaming: true,
            message_type: MessageType::Text,
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("agent_message"));
        assert!(json.contains("streaming"));
    }

    #[test]
    fn test_agent_complete_event() {
        let event = Event::AgentComplete {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            result: AgentResult {
                success: true,
                summary: "Task completed".into(),
                files_changed: vec![],
                output: serde_json::json!({}),
            },
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("agent_complete"));
    }

    #[test]
    fn test_agent_terminated_event() {
        let event = Event::AgentTerminated {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            reason: "User request".into(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("agent_terminated"));
        assert!(json.contains("User request"));
    }

    // === Tool Event Tests ===

    #[test]
    fn test_tool_call_start_event() {
        let event = Event::ToolCallStart {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            call_id: CallId::new(),
            tool_name: "read_file".into(),
            arguments: serde_json::json!({"path": "/tmp/test.txt"}),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("tool_call_start"));
        assert!(json.contains("read_file"));
    }

    #[test]
    fn test_approval_required_event() {
        let event = Event::ApprovalRequired {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            call_id: CallId::new(),
            tool_name: "shell".into(),
            arguments: serde_json::json!({"command": "rm -rf /"}),
            description: "Delete all files".into(),
            risk: RiskLevel::Critical,
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("approval_required"));
        assert!(json.contains("critical"));
        assert!(event.requires_attention());
    }

    #[test]
    fn test_tool_call_complete_event() {
        let event = Event::ToolCallComplete {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            call_id: CallId::new(),
            tool_name: "read_file".into(),
            output: ToolOutput {
                success: true,
                content: "file contents".into(),
                data: None,
                exit_code: Some(0),
            },
            duration_ms: 150,
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("tool_call_complete"));
        assert!(json.contains("duration_ms"));
    }

    #[test]
    fn test_tool_call_failed_event() {
        let event = Event::ToolCallFailed {
            sub_id: SubmissionId::new(),
            agent_id: AgentId::new(),
            call_id: CallId::new(),
            tool_name: "shell".into(),
            error: "Command not found".into(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("tool_call_failed"));
        assert!(json.contains("Command not found"));
    }

    // === Session Event Tests ===

    #[test]
    fn test_session_configured_event() {
        let event = Event::SessionConfigured {
            sub_id: SubmissionId::new(),
            session_id: SessionId::new(),
            config: SessionConfig::default(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("session_configured"));
    }

    #[test]
    fn test_settings_updated_event() {
        let event = Event::SettingsUpdated {
            sub_id: SubmissionId::new(),
            settings: SessionSettings::default(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("settings_updated"));
    }

    // === Checkpoint Event Tests ===

    #[test]
    fn test_checkpoint_saved_event() {
        let event = Event::CheckpointSaved {
            sub_id: SubmissionId::new(),
            checkpoint_id: CheckpointId::new(),
            name: Some("before refactor".into()),
            timestamp: Utc::now(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("checkpoint_saved"));
        assert!(json.contains("before refactor"));
    }

    #[test]
    fn test_checkpoint_restored_event() {
        let event = Event::CheckpointRestored {
            sub_id: SubmissionId::new(),
            checkpoint_id: CheckpointId::new(),
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("checkpoint_restored"));
    }

    #[test]
    fn test_checkpoint_list_event() {
        let event = Event::CheckpointList {
            sub_id: SubmissionId::new(),
            checkpoints: vec![
                CheckpointMeta {
                    id: CheckpointId::new(),
                    name: Some("checkpoint 1".into()),
                    timestamp: Utc::now(),
                    size_bytes: 1024,
                    task_id: None,
                    summary: "First checkpoint".into(),
                },
            ],
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("checkpoint_list"));
        assert!(json.contains("checkpoint 1"));
    }

    // === Plan Event Tests ===

    #[test]
    fn test_plan_mode_changed_event() {
        let event = Event::PlanModeChanged {
            sub_id: SubmissionId::new(),
            enabled: true,
            granularity: PlanGranularity::Detailed,
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("plan_mode_changed"));
        assert!(json.contains("detailed"));
    }

    #[test]
    fn test_plan_created_event() {
        let event = Event::PlanCreated {
            sub_id: SubmissionId::new(),
            plan: TaskPlan {
                original_request: "Add auth".into(),
                steps: vec![
                    PlanStep {
                        id: "1".into(),
                        description: "Create auth module".into(),
                        expected_outcome: "Auth module created".into(),
                        complexity: StepComplexity::Moderate,
                    },
                ],
                agent_assignments: Default::default(),
                dependencies: vec![],
                estimated_tokens: 15000,
            },
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("plan_created"));
        assert!(json.contains("Add auth"));
    }

    // === System Event Tests ===

    #[test]
    fn test_error_event() {
        let event = Event::Error {
            sub_id: SubmissionId::new(),
            message: "Something went wrong".to_string(),
            recoverable: false,
        };
        assert!(event.is_error());
        assert!(event.requires_attention());
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("error"));
        assert!(json.contains("recoverable"));
    }

    #[test]
    fn test_warning_event() {
        let event = Event::Warning {
            sub_id: SubmissionId::new(),
            message: "Rate limit approaching".into(),
            details: Some("80% of limit used".into()),
        };
        
        assert!(!event.is_error());
        assert!(event.requires_attention());
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("warning"));
    }

    #[test]
    fn test_usage_update_event() {
        let event = Event::UsageUpdate {
            sub_id: SubmissionId::new(),
            agent_id: Some(AgentId::new()),
            usage: TokenUsage {
                input_tokens: 1000,
                output_tokens: 500,
                total_tokens: 1500,
                estimated_cost_usd: Some(0.015),
            },
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("usage_update"));
        assert!(json.contains("1500"));
    }

    // === Hierarchy Event Tests ===

    #[test]
    fn test_hierarchy_updated_event() {
        let event = Event::HierarchyUpdated {
            sub_id: SubmissionId::new(),
            root: AgentTree {
                agent_id: AgentId::new(),
                role: AgentRole::Orchestrator,
                status: AgentStatus::Running,
                task_summary: Some("Managing tasks".into()),
                children: vec![
                    AgentTree {
                        agent_id: AgentId::new(),
                        role: AgentRole::Worker,
                        status: AgentStatus::Running,
                        task_summary: Some("Writing code".into()),
                        children: vec![],
                    },
                ],
            },
        };
        
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("hierarchy_updated"));
        assert!(json.contains("orchestrator"));
    }

    // === Sub ID Extraction Tests ===

    #[test]
    fn test_sub_id_extraction_all_events() {
        let events = vec![
            Event::TaskStarted {
                sub_id: SubmissionId::new(),
                task_id: TaskId::new(),
                prompt: "test".into(),
            },
            Event::TaskComplete {
                sub_id: SubmissionId::new(),
                task_id: TaskId::new(),
                result: TaskResult {
                    task_id: TaskId::new(),
                    success: true,
                    summary: "done".into(),
                    files_changed: vec![],
                    token_usage: TokenUsage::default(),
                },
            },
            Event::Error {
                sub_id: SubmissionId::new(),
                message: "error".into(),
                recoverable: false,
            },
            Event::Warning {
                sub_id: SubmissionId::new(),
                message: "warning".into(),
                details: None,
            },
        ];
        
        for event in events {
            let sub_id = event.sub_id();
            assert!(!sub_id.as_str().is_empty());
        }
    }

    // === Error Detection Tests ===

    #[test]
    fn test_is_error_comprehensive() {
        let error_events = vec![
            Event::Error {
                sub_id: SubmissionId::new(),
                message: "error".into(),
                recoverable: false,
            },
            Event::TaskFailed {
                sub_id: SubmissionId::new(),
                task_id: TaskId::new(),
                error: "failed".into(),
            },
        ];
        
        for event in error_events {
            assert!(event.is_error(), "Expected error event");
        }
        
        let non_error_events = vec![
            Event::TaskStarted {
                sub_id: SubmissionId::new(),
                task_id: TaskId::new(),
                prompt: "test".into(),
            },
            Event::Warning {
                sub_id: SubmissionId::new(),
                message: "warning".into(),
                details: None,
            },
        ];
        
        for event in non_error_events {
            assert!(!event.is_error(), "Expected non-error event");
        }
    }

    // === Requires Attention Tests ===

    #[test]
    fn test_requires_attention_comprehensive() {
        let attention_events = vec![
            Event::Error {
                sub_id: SubmissionId::new(),
                message: "error".into(),
                recoverable: false,
            },
            Event::Warning {
                sub_id: SubmissionId::new(),
                message: "warning".into(),
                details: None,
            },
            Event::ApprovalRequired {
                sub_id: SubmissionId::new(),
                agent_id: AgentId::new(),
                call_id: CallId::new(),
                tool_name: "shell".into(),
                arguments: serde_json::json!({}),
                description: "test".into(),
                risk: RiskLevel::High,
            },
        ];
        
        for event in attention_events {
            assert!(event.requires_attention(), "Expected attention-requiring event");
        }
        
        let no_attention_events = vec![
            Event::TaskStarted {
                sub_id: SubmissionId::new(),
                task_id: TaskId::new(),
                prompt: "test".into(),
            },
            Event::AgentMessage {
                sub_id: SubmissionId::new(),
                agent_id: AgentId::new(),
                content: "hello".into(),
                streaming: false,
                message_type: MessageType::Text,
            },
        ];
        
        for event in no_attention_events {
            assert!(!event.requires_attention(), "Expected non-attention event");
        }
    }
}
