use serde::Serialize;

use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "task.engine";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskReq01Create {
    pub team_id: String,
    pub created_by: String,
    pub target_kind: String,
    pub target: String,
    pub title: String,
    pub body: String,
    pub priority: u32,
    pub blocked: bool,
}

impl TaskReq01Create {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Task", "Req", 1, "Create");

    pub fn new(
        team_id: impl Into<String>,
        created_by: impl Into<String>,
        target_kind: impl Into<String>,
        target: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
    ) -> Self {
        Self {
            team_id: team_id.into(),
            created_by: created_by.into(),
            target_kind: target_kind.into(),
            target: target.into(),
            title: title.into(),
            body: body.into(),
            priority: 100,
            blocked: false,
        }
    }

    pub fn queue(self, task_id: impl Into<String>) -> TaskReq02Queued {
        TaskReq02Queued {
            task_id: task_id.into(),
            team_id: self.team_id,
            target_kind: self.target_kind,
            target: self.target,
            priority: self.priority,
            blocked: self.blocked,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskReq02Queued {
    pub task_id: String,
    pub team_id: String,
    pub target_kind: String,
    pub target: String,
    pub priority: u32,
    pub blocked: bool,
}

impl TaskReq02Queued {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Task", "Req", 2, "Queued");

    pub fn dispatch_ready(self) -> TaskReq03DispatchReady {
        TaskReq03DispatchReady {
            task_id: self.task_id,
            team_id: self.team_id,
            target_kind: self.target_kind,
            target: self.target,
            priority: self.priority,
            blocked: self.blocked,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskReq03DispatchReady {
    pub task_id: String,
    pub team_id: String,
    pub target_kind: String,
    pub target: String,
    pub priority: u32,
    pub blocked: bool,
}

impl TaskReq03DispatchReady {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Task", "Req", 3, "DispatchReady");

    pub fn state_changed(
        self,
        status: impl Into<String>,
        event_id: impl Into<String>,
    ) -> TaskResp04StateChanged {
        TaskResp04StateChanged {
            task_id: self.task_id,
            status: status.into(),
            event_id: event_id.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskResp04StateChanged {
    pub task_id: String,
    pub status: String,
    pub event_id: String,
}

impl TaskResp04StateChanged {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Task", "Resp", 4, "StateChanged");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_chain_uses_adjacent_nodes() {
        let changed = TaskReq01Create::new(
            "default",
            "Kevin",
            "role",
            "reviewer",
            "Review",
            "Run checks",
        )
        .queue("AT-000001")
        .dispatch_ready()
        .state_changed("queued", "event-1");

        assert_eq!(changed.task_id, "AT-000001");
        assert_eq!(changed.status, "queued");
        assert_eq!(TaskReq01Create::NODE.number, 1);
        assert_eq!(TaskResp04StateChanged::NODE.number, 4);
    }

    #[test]
    fn task_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "task.engine");
    }
}
