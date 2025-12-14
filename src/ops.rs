//! Operations sent from UI to Goblin orchestrator
//!
//! These are commands that the UI sends to control agent behavior.

use serde::{Deserialize, Serialize};

use crate::ids::*;
use crate::models::*;

/// Operations sent FROM Lair UI TO Goblin orchestrator
///
/// Each operation has an associated `SubmissionId` for correlation with events.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Op {
    /// Configure or reconfigure the session
    ConfigureSession {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// Session configuration
        config: SessionConfig,
    },

    /// Start a new task with user input
    UserInput {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// User's prompt/request
        prompt: String,
        /// Optional images attached to prompt
        #[serde(default)]
        images: Vec<ImageAttachment>,
        /// Optional context to include
        #[serde(default)]
        context: TaskContext,
        /// Resume from specific checkpoint
        #[serde(default)]
        checkpoint_id: Option<CheckpointId>,
    },

    /// Interrupt the current running task
    Interrupt {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// Task to interrupt (None = current task)
        task_id: Option<TaskId>,
    },

    /// Approve or deny a tool execution request
    ExecApproval {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// The call being approved/denied
        call_id: CallId,
        /// Whether to approve
        approved: bool,
        /// Optional modification to command
        #[serde(default)]
        modified_command: Option<String>,
    },

    /// Approve or deny an MCP tool call
    McpApproval {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// The call being approved/denied
        call_id: CallId,
        /// Whether to approve
        approved: bool,
    },

    /// Request to spawn a new agent (typically from orchestrator)
    SpawnAgent {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// Agent configuration
        config: AgentConfig,
        /// Parent agent (None = root orchestrator)
        parent_id: Option<AgentId>,
        /// Task to assign
        task: TaskAssignment,
    },

    /// Terminate a specific agent
    TerminateAgent {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// Agent to terminate
        agent_id: AgentId,
        /// Reason for termination
        #[serde(default)]
        reason: Option<String>,
    },

    /// Send a message to a specific agent
    RouteMessage {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// Target agent
        agent_id: AgentId,
        /// Message content
        content: String,
    },

    /// Save a checkpoint
    SaveCheckpoint {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// Optional name for checkpoint
        #[serde(default)]
        name: Option<String>,
    },

    /// Restore from a checkpoint
    RestoreCheckpoint {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// Checkpoint to restore
        checkpoint_id: CheckpointId,
    },

    /// List available checkpoints
    ListCheckpoints {
        /// Submission ID for correlation
        sub_id: SubmissionId,
    },

    /// Undo to last auto-checkpoint
    Undo {
        /// Submission ID for correlation
        sub_id: SubmissionId,
    },

    /// Toggle plan mode
    TogglePlanMode {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// Enable or disable
        enabled: bool,
        /// Plan granularity
        #[serde(default)]
        granularity: PlanGranularity,
    },

    /// Update session settings
    UpdateSettings {
        /// Submission ID for correlation
        sub_id: SubmissionId,
        /// Settings to update
        settings: SessionSettings,
    },
}

impl Op {
    /// Get the submission ID for this operation
    pub fn sub_id(&self) -> &SubmissionId {
        match self {
            Op::ConfigureSession { sub_id, .. } => sub_id,
            Op::UserInput { sub_id, .. } => sub_id,
            Op::Interrupt { sub_id, .. } => sub_id,
            Op::ExecApproval { sub_id, .. } => sub_id,
            Op::McpApproval { sub_id, .. } => sub_id,
            Op::SpawnAgent { sub_id, .. } => sub_id,
            Op::TerminateAgent { sub_id, .. } => sub_id,
            Op::RouteMessage { sub_id, .. } => sub_id,
            Op::SaveCheckpoint { sub_id, .. } => sub_id,
            Op::RestoreCheckpoint { sub_id, .. } => sub_id,
            Op::ListCheckpoints { sub_id, .. } => sub_id,
            Op::Undo { sub_id, .. } => sub_id,
            Op::TogglePlanMode { sub_id, .. } => sub_id,
            Op::UpdateSettings { sub_id, .. } => sub_id,
        }
    }

    /// Create a UserInput operation
    pub fn user_input(prompt: impl Into<String>) -> Self {
        Op::UserInput {
            sub_id: SubmissionId::new(),
            prompt: prompt.into(),
            images: vec![],
            context: TaskContext::default(),
            checkpoint_id: None,
        }
    }

    /// Create an Interrupt operation
    pub fn interrupt() -> Self {
        Op::Interrupt {
            sub_id: SubmissionId::new(),
            task_id: None,
        }
    }

    /// Create an ExecApproval operation
    pub fn approve_exec(call_id: CallId) -> Self {
        Op::ExecApproval {
            sub_id: SubmissionId::new(),
            call_id,
            approved: true,
            modified_command: None,
        }
    }

    /// Create a deny ExecApproval operation
    pub fn deny_exec(call_id: CallId) -> Self {
        Op::ExecApproval {
            sub_id: SubmissionId::new(),
            call_id,
            approved: false,
            modified_command: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === UserInput Operation Tests ===

    #[test]
    fn test_user_input_serialization() {
        let op = Op::user_input("Hello, world!");
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("user_input"));
        assert!(json.contains("Hello, world!"));

        let parsed: Op = serde_json::from_str(&json).unwrap();
        match parsed {
            Op::UserInput { prompt, .. } => assert_eq!(prompt, "Hello, world!"),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_user_input_has_unique_sub_id() {
        let op1 = Op::user_input("test1");
        let op2 = Op::user_input("test2");
        assert_ne!(op1.sub_id().as_str(), op2.sub_id().as_str());
    }

    #[test]
    fn test_user_input_defaults() {
        let op = Op::user_input("test");
        match op {
            Op::UserInput { images, context, checkpoint_id, .. } => {
                assert!(images.is_empty());
                assert!(context.files.is_empty());
                assert!(checkpoint_id.is_none());
            }
            _ => panic!("Wrong variant"),
        }
    }

    // === Interrupt Operation Tests ===

    #[test]
    fn test_interrupt_op() {
        let op = Op::interrupt();
        let sub_id = op.sub_id();
        assert!(!sub_id.as_str().is_empty());
        
        match op {
            Op::Interrupt { task_id, .. } => assert!(task_id.is_none()),
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_interrupt_serialization() {
        let op = Op::interrupt();
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("interrupt"));
        
        let parsed: Op = serde_json::from_str(&json).unwrap();
        match parsed {
            Op::Interrupt { task_id, .. } => assert!(task_id.is_none()),
            _ => panic!("Wrong variant"),
        }
    }

    // === ExecApproval Operation Tests ===

    #[test]
    fn test_approve_exec() {
        let call_id = CallId::new();
        let op = Op::approve_exec(call_id);
        
        match op {
            Op::ExecApproval { approved, modified_command, call_id: cid, .. } => {
                assert!(approved);
                assert!(modified_command.is_none());
                assert_eq!(cid, call_id);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_deny_exec() {
        let call_id = CallId::new();
        let op = Op::deny_exec(call_id);
        
        match op {
            Op::ExecApproval { approved, .. } => {
                assert!(!approved);
            }
            _ => panic!("Wrong variant"),
        }
    }

    #[test]
    fn test_exec_approval_serialization() {
        let call_id = CallId::new();
        let op = Op::approve_exec(call_id);
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("exec_approval"));
        assert!(json.contains("approved"));
    }

    // === ConfigureSession Operation Tests ===

    #[test]
    fn test_configure_session() {
        let config = SessionConfig::default();
        let op = Op::ConfigureSession {
            sub_id: SubmissionId::new(),
            config: config.clone(),
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("configure_session"));
        
        let parsed: Op = serde_json::from_str(&json).unwrap();
        match parsed {
            Op::ConfigureSession { config: c, .. } => {
                assert_eq!(c.max_parallel_agents, config.max_parallel_agents);
            }
            _ => panic!("Wrong variant"),
        }
    }

    // === SpawnAgent Operation Tests ===

    #[test]
    fn test_spawn_agent() {
        let task = TaskAssignment {
            task_id: TaskId::new(),
            description: "Test task".into(),
            deliverables: vec!["result".into()],
            dependencies: vec![],
            context: TaskContext::default(),
        };
        
        let op = Op::SpawnAgent {
            sub_id: SubmissionId::new(),
            config: AgentConfig::default(),
            parent_id: None,
            task,
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("spawn_agent"));
        assert!(json.contains("Test task"));
    }

    // === TerminateAgent Operation Tests ===

    #[test]
    fn test_terminate_agent() {
        let agent_id = AgentId::new();
        let op = Op::TerminateAgent {
            sub_id: SubmissionId::new(),
            agent_id,
            reason: Some("Test termination".into()),
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("terminate_agent"));
        assert!(json.contains("Test termination"));
    }

    // === Checkpoint Operations Tests ===

    #[test]
    fn test_save_checkpoint() {
        let op = Op::SaveCheckpoint {
            sub_id: SubmissionId::new(),
            name: Some("manual checkpoint".into()),
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("save_checkpoint"));
        assert!(json.contains("manual checkpoint"));
    }

    #[test]
    fn test_restore_checkpoint() {
        let checkpoint_id = CheckpointId::new();
        let op = Op::RestoreCheckpoint {
            sub_id: SubmissionId::new(),
            checkpoint_id,
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("restore_checkpoint"));
    }

    #[test]
    fn test_list_checkpoints() {
        let op = Op::ListCheckpoints {
            sub_id: SubmissionId::new(),
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("list_checkpoints"));
    }

    #[test]
    fn test_undo() {
        let op = Op::Undo {
            sub_id: SubmissionId::new(),
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("undo"));
    }

    // === Plan Mode Operations Tests ===

    #[test]
    fn test_toggle_plan_mode() {
        let op = Op::TogglePlanMode {
            sub_id: SubmissionId::new(),
            enabled: true,
            granularity: PlanGranularity::Detailed,
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("toggle_plan_mode"));
        assert!(json.contains("detailed"));
    }

    // === Update Settings Tests ===

    #[test]
    fn test_update_settings() {
        let settings = SessionSettings {
            show_rate_limit: true,
            subagent_concurrency: Some(4),
            plan_granularity: PlanGranularity::Auto,
        };
        
        let op = Op::UpdateSettings {
            sub_id: SubmissionId::new(),
            settings,
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("update_settings"));
        assert!(json.contains("show_rate_limit"));
    }

    // === MCP Approval Tests ===

    #[test]
    fn test_mcp_approval() {
        let call_id = CallId::new();
        let op = Op::McpApproval {
            sub_id: SubmissionId::new(),
            call_id,
            approved: true,
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("mcp_approval"));
    }

    // === Route Message Tests ===

    #[test]
    fn test_route_message() {
        let agent_id = AgentId::new();
        let op = Op::RouteMessage {
            sub_id: SubmissionId::new(),
            agent_id,
            content: "Hello agent!".into(),
        };
        
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("route_message"));
        assert!(json.contains("Hello agent!"));
    }

    // === Sub ID Extraction Tests ===

    #[test]
    fn test_sub_id_extraction_all_variants() {
        // Test that sub_id() works for all variants
        let ops = vec![
            Op::user_input("test"),
            Op::interrupt(),
            Op::approve_exec(CallId::new()),
            Op::ConfigureSession {
                sub_id: SubmissionId::new(),
                config: SessionConfig::default(),
            },
            Op::SaveCheckpoint {
                sub_id: SubmissionId::new(),
                name: None,
            },
            Op::ListCheckpoints {
                sub_id: SubmissionId::new(),
            },
            Op::Undo {
                sub_id: SubmissionId::new(),
            },
            Op::TogglePlanMode {
                sub_id: SubmissionId::new(),
                enabled: true,
                granularity: PlanGranularity::Auto,
            },
        ];
        
        for op in ops {
            let sub_id = op.sub_id();
            assert!(!sub_id.as_str().is_empty(), "sub_id should not be empty");
        }
    }
}
