//! Data models used across the protocol

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

use crate::ids::*;

// === Session Configuration ===

/// Configuration for a Goblin session
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Working directory for agents
    #[serde(default)]
    pub cwd: Option<PathBuf>,
    /// Model to use for orchestrator
    #[serde(default)]
    pub model: Option<String>,
    /// Custom system instructions
    #[serde(default)]
    pub instructions: Option<String>,
    /// MCP servers to connect
    #[serde(default)]
    pub mcp_servers: Vec<McpServerConfig>,
    /// Approval mode
    #[serde(default)]
    pub approval_mode: ApprovalMode,
    /// Sandbox policy
    #[serde(default)]
    pub sandbox: SandboxConfig,
    /// Max parallel agents
    #[serde(default = "default_max_agents")]
    pub max_parallel_agents: usize,
}

fn default_max_agents() -> usize {
    8
}

/// Session runtime settings (modifiable without reconfigure)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionSettings {
    /// Show rate limit info in UI
    #[serde(default)]
    pub show_rate_limit: bool,
    /// Number of parallel subagents
    #[serde(default)]
    pub subagent_concurrency: Option<usize>,
    /// Plan mode granularity
    #[serde(default)]
    pub plan_granularity: PlanGranularity,
}

/// Approval mode for tool execution
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalMode {
    /// Always require approval
    Always,
    /// Never require approval (dangerous!)
    Never,
    /// Require approval based on risk level
    #[default]
    RiskBased,
    /// Custom rules (defined in policy)
    Custom,
}

/// Sandbox configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Enable sandboxing
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Network access policy
    #[serde(default)]
    pub network: NetworkPolicy,
    /// Additional writable paths
    #[serde(default)]
    pub writable_paths: Vec<PathBuf>,
    /// Execution timeout
    #[serde(default)]
    pub timeout_secs: Option<u64>,
}

fn default_true() -> bool {
    true
}

/// Network access policy for sandbox
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkPolicy {
    /// No network access
    #[default]
    None,
    /// Localhost only
    Localhost,
    /// Specific hosts allowed
    Allowlist(Vec<String>),
    /// Full network access
    Full,
}

// === MCP Configuration ===

/// Configuration for an MCP server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Unique identifier for this server
    pub id: String,
    /// Display name
    pub name: String,
    /// Transport type
    pub transport: McpTransport,
    /// Environment variables to set
    #[serde(default)]
    pub env: HashMap<String, String>,
}

/// MCP transport configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum McpTransport {
    /// stdio-based transport
    Stdio {
        command: String,
        #[serde(default)]
        args: Vec<String>,
    },
    /// Socket-based transport
    Socket {
        path: PathBuf,
    },
    /// HTTP/SSE transport
    Http {
        url: String,
    },
}

// === Agent Types ===

/// Role of an agent in the hierarchy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentRole {
    /// Top-level orchestrator
    Orchestrator,
    /// Domain lead (e.g., Frontend Lead, Backend Lead)
    DomainLead { domain: String },
    /// Worker agent
    Worker,
    /// Specialist agent (e.g., Security, Performance)
    Specialist { specialty: String },
    /// Research/exploration agent
    Scout,
    /// Code review agent
    Reviewer,
    /// Custom role
    Custom { name: String },
}

impl Default for AgentRole {
    fn default() -> Self {
        AgentRole::Worker
    }
}

/// Configuration for spawning an agent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Agent role
    #[serde(default)]
    pub role: AgentRole,
    /// Model to use
    #[serde(default)]
    pub model: Option<String>,
    /// Working directory
    #[serde(default)]
    pub cwd: Option<PathBuf>,
    /// Git worktree (for isolation)
    #[serde(default)]
    pub worktree: Option<String>,
    /// Tools available to this agent
    #[serde(default)]
    pub tools: Vec<String>,
    /// Can this agent spawn sub-agents?
    #[serde(default)]
    pub can_spawn: bool,
    /// Max sub-agents this agent can spawn
    #[serde(default)]
    pub max_children: Option<usize>,
    /// Token budget for this agent
    #[serde(default)]
    pub token_budget: Option<u64>,
}

/// Current status of an agent
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    /// Being created
    Spawning,
    /// Loading context
    Initializing,
    /// Actively working
    Running,
    /// Waiting for input/approval/dependency
    Waiting { reason: String },
    /// Task completed
    Completed,
    /// Task failed
    Failed,
    /// Manually terminated
    Terminated,
}

impl Default for AgentStatus {
    fn default() -> Self {
        AgentStatus::Spawning
    }
}

/// Result from an agent completing its task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    /// Success or failure
    pub success: bool,
    /// Summary of what was done
    pub summary: String,
    /// Files changed
    #[serde(default)]
    pub files_changed: Vec<PathBuf>,
    /// Output data (structured)
    #[serde(default)]
    pub output: serde_json::Value,
}

// === Task Types ===

/// Context provided with a task
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskContext {
    /// Current working directory
    #[serde(default)]
    pub cwd: Option<PathBuf>,
    /// Files to include as context
    #[serde(default)]
    pub files: Vec<PathBuf>,
    /// Additional context from Grimoire
    #[serde(default)]
    pub memory_context: Vec<String>,
    /// Custom metadata
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Task assigned to an agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAssignment {
    /// Task ID
    pub task_id: TaskId,
    /// Task description
    pub description: String,
    /// Expected deliverables
    #[serde(default)]
    pub deliverables: Vec<String>,
    /// Dependencies on other tasks
    #[serde(default)]
    pub dependencies: Vec<TaskId>,
    /// Context for this task
    #[serde(default)]
    pub context: TaskContext,
}

/// Result of a completed task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Task ID
    pub task_id: TaskId,
    /// Success or failure
    pub success: bool,
    /// Summary
    pub summary: String,
    /// Files changed
    #[serde(default)]
    pub files_changed: Vec<PathBuf>,
    /// Token usage
    #[serde(default)]
    pub token_usage: TokenUsage,
}

// === Tool Types ===

/// Output from a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    /// Success or failure
    pub success: bool,
    /// Output content
    pub content: String,
    /// Structured data
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    /// Exit code (for shell commands)
    #[serde(default)]
    pub exit_code: Option<i32>,
}

/// Risk level for tool execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    /// No risk (read-only)
    None,
    /// Low risk (local changes)
    Low,
    /// Medium risk (file modifications)
    Medium,
    /// High risk (destructive or network)
    High,
    /// Critical risk (system-level)
    Critical,
}

impl Default for RiskLevel {
    fn default() -> Self {
        RiskLevel::Medium
    }
}

// === Plan Types ===

/// Granularity of task planning
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanGranularity {
    /// High-level steps only
    Coarse,
    /// Detailed implementation plan
    Detailed,
    /// Let the model decide
    #[default]
    Auto,
}

/// A task decomposition plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPlan {
    /// Original request
    pub original_request: String,
    /// Decomposed steps
    pub steps: Vec<PlanStep>,
    /// Agent assignments
    pub agent_assignments: HashMap<String, AgentRole>,
    /// Dependency graph
    pub dependencies: Vec<(String, String)>,
    /// Estimated token usage
    #[serde(default)]
    pub estimated_tokens: u64,
}

/// A single step in a plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    /// Step ID
    pub id: String,
    /// Step description
    pub description: String,
    /// Expected outcome
    pub expected_outcome: String,
    /// Estimated complexity
    #[serde(default)]
    pub complexity: StepComplexity,
}

/// Complexity of a plan step
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepComplexity {
    /// Simple, single action
    Simple,
    /// Moderate complexity
    #[default]
    Moderate,
    /// Complex, may need decomposition
    Complex,
}

// === Hierarchy Types ===

/// Tree representation of agent hierarchy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTree {
    /// Agent info
    pub agent_id: AgentId,
    pub role: AgentRole,
    pub status: AgentStatus,
    pub task_summary: Option<String>,
    /// Children in hierarchy
    #[serde(default)]
    pub children: Vec<AgentTree>,
}

// === Checkpoint Types ===

/// Metadata for a checkpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointMeta {
    /// Checkpoint ID
    pub id: CheckpointId,
    /// Optional name
    pub name: Option<String>,
    /// When created
    pub timestamp: DateTime<Utc>,
    /// Size in bytes
    pub size_bytes: u64,
    /// Task ID at checkpoint
    pub task_id: Option<TaskId>,
    /// Summary
    pub summary: String,
}

// === Usage Types ===

/// Token usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Input tokens
    pub input_tokens: u64,
    /// Output tokens
    pub output_tokens: u64,
    /// Total tokens
    pub total_tokens: u64,
    /// Estimated cost (USD)
    #[serde(default)]
    pub estimated_cost_usd: Option<f64>,
}

// === Message Types ===

/// Type of message from an agent
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// Regular text response
    Text,
    /// Thinking/reasoning (may be hidden)
    Thinking,
    /// Code block
    Code,
    /// Error message
    Error,
    /// Status update
    Status,
    /// Progress indicator
    Progress,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::Text
    }
}

// === Attachment Types ===

/// Image attached to a prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAttachment {
    /// Base64 encoded image data
    pub data: String,
    /// MIME type
    pub mime_type: String,
    /// Optional filename
    #[serde(default)]
    pub filename: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // === SessionConfig Tests ===

    #[test]
    fn test_session_config_defaults() {
        let config: SessionConfig = serde_json::from_str("{}").unwrap();
        assert_eq!(config.max_parallel_agents, 8);
        assert_eq!(config.approval_mode, ApprovalMode::RiskBased);
        // Note: sandbox uses struct Default which gives enabled=false
        // The serde default_true only applies when deserializing SandboxConfig directly
    }

    #[test]
    fn test_session_config_custom_values() {
        let json = r#"{
            "max_parallel_agents": 16,
            "approval_mode": "always",
            "model": "claude-3-opus"
        }"#;
        let config: SessionConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.max_parallel_agents, 16);
        assert_eq!(config.approval_mode, ApprovalMode::Always);
        assert_eq!(config.model, Some("claude-3-opus".into()));
    }

    #[test]
    fn test_session_config_with_mcp_servers() {
        let config = SessionConfig {
            mcp_servers: vec![
                McpServerConfig {
                    id: "my-server".into(),
                    name: "My Server".into(),
                    transport: McpTransport::Stdio {
                        command: "node".into(),
                        args: vec!["server.js".into()],
                    },
                    env: Default::default(),
                },
            ],
            ..Default::default()
        };
        
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("my-server"));
        assert!(json.contains("My Server"));
    }

    // === ApprovalMode Tests ===

    #[test]
    fn test_approval_mode_serialization() {
        let modes = vec![
            (ApprovalMode::Always, "always"),
            (ApprovalMode::Never, "never"),
            (ApprovalMode::RiskBased, "risk_based"),
            (ApprovalMode::Custom, "custom"),
        ];
        
        for (mode, expected) in modes {
            let json = serde_json::to_string(&mode).unwrap();
            assert!(json.contains(expected));
            
            let parsed: ApprovalMode = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, mode);
        }
    }

    // === SandboxConfig Tests ===

    #[test]
    fn test_sandbox_config_defaults() {
        let config: SandboxConfig = serde_json::from_str("{}").unwrap();
        assert!(config.enabled);
        assert_eq!(config.network, NetworkPolicy::None);
        assert!(config.writable_paths.is_empty());
    }

    #[test]
    fn test_sandbox_config_custom() {
        let config = SandboxConfig {
            enabled: true,
            network: NetworkPolicy::Localhost,
            writable_paths: vec![PathBuf::from("/tmp")],
            timeout_secs: Some(120),
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let parsed: SandboxConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.network, NetworkPolicy::Localhost);
        assert_eq!(parsed.timeout_secs, Some(120));
    }

    // === NetworkPolicy Tests ===

    #[test]
    fn test_network_policy_variants() {
        let policies = vec![
            NetworkPolicy::None,
            NetworkPolicy::Localhost,
            NetworkPolicy::Full,
            NetworkPolicy::Allowlist(vec!["api.example.com".into()]),
        ];
        
        for policy in policies {
            let json = serde_json::to_string(&policy).unwrap();
            let parsed: NetworkPolicy = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, policy);
        }
    }

    // === McpServerConfig Tests ===

    #[test]
    fn test_mcp_server_stdio_transport() {
        let config = McpServerConfig {
            id: "test".into(),
            name: "Test Server".into(),
            transport: McpTransport::Stdio {
                command: "npx".into(),
                args: vec!["my-server".into()],
            },
            env: [("API_KEY".into(), "secret".into())].into(),
        };
        
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("stdio"));
        assert!(json.contains("npx"));
    }

    #[test]
    fn test_mcp_server_socket_transport() {
        let config = McpServerConfig {
            id: "test".into(),
            name: "Test Server".into(),
            transport: McpTransport::Socket {
                path: PathBuf::from("/var/run/mcp.sock"),
            },
            env: Default::default(),
        };
        
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("socket"));
        assert!(json.contains("mcp.sock"));
    }

    #[test]
    fn test_mcp_server_http_transport() {
        let config = McpServerConfig {
            id: "test".into(),
            name: "Test Server".into(),
            transport: McpTransport::Http {
                url: "http://localhost:3000".into(),
            },
            env: Default::default(),
        };
        
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("http"));
        assert!(json.contains("localhost:3000"));
    }

    // === AgentRole Tests ===

    #[test]
    fn test_agent_role_serialization() {
        let role = AgentRole::DomainLead { domain: "frontend".to_string() };
        let json = serde_json::to_string(&role).unwrap();
        assert!(json.contains("domain_lead"));
        assert!(json.contains("frontend"));
    }

    #[test]
    fn test_agent_role_variants() {
        let roles = vec![
            AgentRole::Orchestrator,
            AgentRole::Worker,
            AgentRole::Scout,
            AgentRole::Reviewer,
            AgentRole::DomainLead { domain: "backend".into() },
            AgentRole::Specialist { specialty: "security".into() },
            AgentRole::Custom { name: "my-role".into() },
        ];
        
        for role in roles {
            let json = serde_json::to_string(&role).unwrap();
            let parsed: AgentRole = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, role);
        }
    }

    #[test]
    fn test_agent_role_default() {
        let role: AgentRole = Default::default();
        assert_eq!(role, AgentRole::Worker);
    }

    // === AgentConfig Tests ===

    #[test]
    fn test_agent_config_defaults() {
        let config: AgentConfig = Default::default();
        assert_eq!(config.role, AgentRole::Worker);
        assert!(!config.can_spawn);
        assert!(config.model.is_none());
    }

    #[test]
    fn test_agent_config_custom() {
        let config = AgentConfig {
            role: AgentRole::Orchestrator,
            model: Some("claude-3-opus".into()),
            cwd: Some(PathBuf::from("/home/user/project")),
            can_spawn: true,
            max_children: Some(4),
            token_budget: Some(100_000),
            tools: vec!["read_file".into(), "write_file".into()],
            worktree: None,
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let parsed: AgentConfig = serde_json::from_str(&json).unwrap();
        
        assert!(parsed.can_spawn);
        assert_eq!(parsed.max_children, Some(4));
        assert_eq!(parsed.token_budget, Some(100_000));
    }

    // === AgentStatus Tests ===

    #[test]
    fn test_agent_status_variants() {
        let statuses = vec![
            AgentStatus::Spawning,
            AgentStatus::Initializing,
            AgentStatus::Running,
            AgentStatus::Waiting { reason: "Waiting for approval".into() },
            AgentStatus::Completed,
            AgentStatus::Failed,
            AgentStatus::Terminated,
        ];
        
        for status in statuses {
            let json = serde_json::to_string(&status).unwrap();
            let parsed: AgentStatus = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, status);
        }
    }

    #[test]
    fn test_agent_status_default() {
        let status: AgentStatus = Default::default();
        assert_eq!(status, AgentStatus::Spawning);
    }

    // === AgentResult Tests ===

    #[test]
    fn test_agent_result() {
        let result = AgentResult {
            success: true,
            summary: "Task completed successfully".into(),
            files_changed: vec![PathBuf::from("src/main.rs")],
            output: serde_json::json!({"lines_added": 50}),
        };
        
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("Task completed successfully"));
        assert!(json.contains("src/main.rs"));
    }

    // === TaskContext Tests ===

    #[test]
    fn test_task_context_defaults() {
        let ctx: TaskContext = Default::default();
        assert!(ctx.cwd.is_none());
        assert!(ctx.files.is_empty());
        assert!(ctx.memory_context.is_empty());
    }

    #[test]
    fn test_task_context_custom() {
        let ctx = TaskContext {
            cwd: Some(PathBuf::from("/project")),
            files: vec![PathBuf::from("src/lib.rs")],
            memory_context: vec!["Previous context".into()],
            metadata: [("key".into(), serde_json::json!("value"))].into(),
        };
        
        let json = serde_json::to_string(&ctx).unwrap();
        let parsed: TaskContext = serde_json::from_str(&json).unwrap();
        
        assert_eq!(parsed.cwd, Some(PathBuf::from("/project")));
        assert!(!parsed.files.is_empty());
    }

    // === TaskAssignment Tests ===

    #[test]
    fn test_task_assignment() {
        let dep_id = TaskId::new();
        let task = TaskAssignment {
            task_id: TaskId::new(),
            description: "Implement feature X".into(),
            deliverables: vec!["Code".into(), "Tests".into()],
            dependencies: vec![dep_id],
            context: TaskContext::default(),
        };
        
        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("Implement feature X"));
        assert!(json.contains("deliverables"));
    }

    // === TaskResult Tests ===

    #[test]
    fn test_task_result() {
        let result = TaskResult {
            task_id: TaskId::new(),
            success: true,
            summary: "Done".into(),
            files_changed: vec![PathBuf::from("a.rs"), PathBuf::from("b.rs")],
            token_usage: TokenUsage {
                input_tokens: 5000,
                output_tokens: 2000,
                total_tokens: 7000,
                estimated_cost_usd: Some(0.07),
            },
        };
        
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("7000"));
        assert!(json.contains("0.07"));
    }

    // === ToolOutput Tests ===

    #[test]
    fn test_tool_output() {
        let output = ToolOutput {
            success: true,
            content: "File read successfully".into(),
            data: Some(serde_json::json!({"lines": 100})),
            exit_code: Some(0),
        };
        
        let json = serde_json::to_string(&output).unwrap();
        assert!(json.contains("exit_code"));
    }

    // === RiskLevel Tests ===

    #[test]
    fn test_risk_level_variants() {
        let levels = vec![
            RiskLevel::None,
            RiskLevel::Low,
            RiskLevel::Medium,
            RiskLevel::High,
            RiskLevel::Critical,
        ];
        
        for level in levels {
            let json = serde_json::to_string(&level).unwrap();
            let parsed: RiskLevel = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, level);
        }
    }

    #[test]
    fn test_risk_level_default() {
        let level: RiskLevel = Default::default();
        assert_eq!(level, RiskLevel::Medium);
    }

    // === PlanGranularity Tests ===

    #[test]
    fn test_plan_granularity_variants() {
        let granularities = vec![
            PlanGranularity::Coarse,
            PlanGranularity::Detailed,
            PlanGranularity::Auto,
        ];
        
        for g in granularities {
            let json = serde_json::to_string(&g).unwrap();
            let parsed: PlanGranularity = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, g);
        }
    }

    #[test]
    fn test_plan_granularity_default() {
        let g: PlanGranularity = Default::default();
        assert_eq!(g, PlanGranularity::Auto);
    }

    // === TaskPlan Tests ===

    #[test]
    fn test_task_plan() {
        let plan = TaskPlan {
            original_request: "Add authentication".into(),
            steps: vec![
                PlanStep {
                    id: "1".into(),
                    description: "Create auth module".into(),
                    expected_outcome: "Working auth".into(),
                    complexity: StepComplexity::Complex,
                },
                PlanStep {
                    id: "2".into(),
                    description: "Add tests".into(),
                    expected_outcome: "Tests passing".into(),
                    complexity: StepComplexity::Moderate,
                },
            ],
            agent_assignments: [("1".into(), AgentRole::Worker)].into(),
            dependencies: vec![("1".into(), "2".into())],
            estimated_tokens: 30_000,
        };
        
        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains("Add authentication"));
        assert!(json.contains("30000"));
    }

    // === StepComplexity Tests ===

    #[test]
    fn test_step_complexity_variants() {
        let complexities = vec![
            StepComplexity::Simple,
            StepComplexity::Moderate,
            StepComplexity::Complex,
        ];
        
        for c in complexities {
            let json = serde_json::to_string(&c).unwrap();
            let parsed: StepComplexity = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, c);
        }
    }

    #[test]
    fn test_step_complexity_default() {
        let c: StepComplexity = Default::default();
        assert_eq!(c, StepComplexity::Moderate);
    }

    // === AgentTree Tests ===

    #[test]
    fn test_agent_tree_nested() {
        let tree = AgentTree {
            agent_id: AgentId::new(),
            role: AgentRole::Orchestrator,
            status: AgentStatus::Running,
            task_summary: Some("Managing".into()),
            children: vec![
                AgentTree {
                    agent_id: AgentId::new(),
                    role: AgentRole::Worker,
                    status: AgentStatus::Running,
                    task_summary: Some("Coding".into()),
                    children: vec![],
                },
                AgentTree {
                    agent_id: AgentId::new(),
                    role: AgentRole::Worker,
                    status: AgentStatus::Waiting { reason: "Blocked".into() },
                    task_summary: Some("Testing".into()),
                    children: vec![],
                },
            ],
        };
        
        let json = serde_json::to_string(&tree).unwrap();
        assert!(json.contains("orchestrator"));
        assert!(json.contains("children"));
    }

    // === CheckpointMeta Tests ===

    #[test]
    fn test_checkpoint_meta() {
        use chrono::Utc;
        
        let meta = CheckpointMeta {
            id: CheckpointId::new(),
            name: Some("Before refactor".into()),
            timestamp: Utc::now(),
            size_bytes: 1024 * 1024,
            task_id: Some(TaskId::new()),
            summary: "Checkpoint before major changes".into(),
        };
        
        let json = serde_json::to_string(&meta).unwrap();
        assert!(json.contains("Before refactor"));
        assert!(json.contains("1048576"));
    }

    // === TokenUsage Tests ===

    #[test]
    fn test_token_usage_default() {
        let usage: TokenUsage = Default::default();
        assert_eq!(usage.input_tokens, 0);
        assert_eq!(usage.output_tokens, 0);
        assert_eq!(usage.total_tokens, 0);
        assert!(usage.estimated_cost_usd.is_none());
    }

    #[test]
    fn test_token_usage_with_cost() {
        let usage = TokenUsage {
            input_tokens: 10_000,
            output_tokens: 5_000,
            total_tokens: 15_000,
            estimated_cost_usd: Some(0.15),
        };
        
        let json = serde_json::to_string(&usage).unwrap();
        assert!(json.contains("15000"));
        assert!(json.contains("0.15"));
    }

    // === MessageType Tests ===

    #[test]
    fn test_message_type_variants() {
        let types = vec![
            MessageType::Text,
            MessageType::Thinking,
            MessageType::Code,
            MessageType::Error,
            MessageType::Status,
            MessageType::Progress,
        ];
        
        for t in types {
            let json = serde_json::to_string(&t).unwrap();
            let parsed: MessageType = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, t);
        }
    }

    #[test]
    fn test_message_type_default() {
        let t: MessageType = Default::default();
        assert_eq!(t, MessageType::Text);
    }

    // === ImageAttachment Tests ===

    #[test]
    fn test_image_attachment() {
        let attachment = ImageAttachment {
            data: "iVBORw0KGgo=".into(),
            mime_type: "image/png".into(),
            filename: Some("screenshot.png".into()),
        };
        
        let json = serde_json::to_string(&attachment).unwrap();
        assert!(json.contains("image/png"));
        assert!(json.contains("screenshot.png"));
    }

    // === SessionSettings Tests ===

    #[test]
    fn test_session_settings_defaults() {
        let settings: SessionSettings = Default::default();
        assert!(!settings.show_rate_limit);
        assert!(settings.subagent_concurrency.is_none());
        assert_eq!(settings.plan_granularity, PlanGranularity::Auto);
    }

    #[test]
    fn test_session_settings_custom() {
        let settings = SessionSettings {
            show_rate_limit: true,
            subagent_concurrency: Some(8),
            plan_granularity: PlanGranularity::Detailed,
        };
        
        let json = serde_json::to_string(&settings).unwrap();
        let parsed: SessionSettings = serde_json::from_str(&json).unwrap();
        
        assert!(parsed.show_rate_limit);
        assert_eq!(parsed.subagent_concurrency, Some(8));
    }
}
