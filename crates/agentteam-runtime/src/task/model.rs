use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Queued,
    Running,
    Done,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskTargetKind {
    Agent,
    Role,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskCreateInput {
    pub team_id: String,
    pub created_by: String,
    pub target_kind: TaskTargetKind,
    pub target: String,
    pub title: String,
    pub body: String,
    pub priority: u32,
    pub blocked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskClaimInput {
    pub worker_name: String,
    pub worker_role: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskTransitionInput {
    pub task_id: String,
    pub actor: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskEventPayload {
    pub task_id: String,
    pub team_id: String,
    pub actor: String,
    pub status: TaskStatus,
    pub target_kind: TaskTargetKind,
    pub target: String,
    pub title: String,
    pub body: String,
    #[serde(default = "default_task_priority")]
    pub priority: u32,
    #[serde(default)]
    pub blocked: bool,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskRecord {
    pub task_id: String,
    pub team_id: String,
    pub status: TaskStatus,
    pub target_kind: TaskTargetKind,
    pub target: String,
    pub title: String,
    pub body: String,
    pub priority: u32,
    pub blocked: bool,
    pub created_by: String,
    pub latest_actor: String,
    pub latest_detail: String,
    pub latest_event_id: String,
    pub latest_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoard {
    pub task_count: usize,
    pub tasks: Vec<TaskRecord>,
    pub latest_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskStateChanged {
    pub task_id: String,
    pub status: TaskStatus,
    pub event_id: String,
    pub sequence: u64,
}

impl TaskStatus {
    pub fn label(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Done => "done",
            Self::Error => "error",
        }
    }
}

impl TaskTargetKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Role => "role",
        }
    }
}

fn default_task_priority() -> u32 {
    100
}
