use std::path::PathBuf;

use agentteam_config::NormalizedConfig;
use agentteam_contracts::debug::DebugResp03Bundle;
use agentteam_tmux::TmuxLoopbackReport;
use serde::Serialize;

use crate::domain::{
    DomainRegistrySnapshot, DomainRouteKind, DomainTargetKind, ResolvedDomainTarget,
};
use crate::task::{TaskBoard, TaskStateChanged};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocalCommandError {
    Config { reason: String },
    Domain { reason: String },
    Debug { reason: String },
    Task { reason: String },
    Tmux { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum LocalCommandResult {
    ConfigCheck {
        normalized: ConfigCheckResult,
    },
    DaemonCheck {
        daemon: DaemonCheckResult,
    },
    DomainResolve {
        target: ResolvedDomainTargetResult,
        registry_snapshot: DomainRegistrySnapshotResult,
    },
    DebugSnapshot {
        bundle: DebugBundleResult,
    },
    TaskSend {
        task: TaskStateChangedResult,
    },
    TaskList {
        board: TaskBoardResult,
    },
    TaskStatus {
        board: TaskBoardResult,
    },
    TaskDone {
        task: TaskStateChangedResult,
    },
    TaskError {
        task: TaskStateChangedResult,
    },
    TaskClaim {
        task: TaskStateChangedResult,
    },
    TmuxLoopback {
        loopback: TmuxLoopbackResult,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConfigCheckResult {
    pub path: String,
    pub project_slug: String,
    pub project_root: String,
    pub runtime_home: String,
    pub local_domain_id: String,
    pub team_count: usize,
    pub member_count: usize,
    pub zterm_endpoint: String,
    pub remote_domain_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DaemonCheckResult {
    pub project_slug: String,
    pub runtime_home: String,
    pub local_domain_id: String,
    pub routeable_endpoint_count: usize,
    pub config_status: String,
    pub domain_registry_status: String,
    pub daemon_process_status: String,
    pub tmux_status: String,
    pub zterm_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResolvedDomainTargetResult {
    pub original_target: String,
    pub target_kind: String,
    pub target_value: String,
    pub domain_id: String,
    pub route_kind: String,
    pub endpoint_host: String,
    pub endpoint_port: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DomainRegistrySnapshotResult {
    pub local_domain_id: String,
    pub aliases: Vec<String>,
    pub remote_domain_ids: Vec<String>,
    pub endpoint_count: usize,
    pub token_redaction_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DebugBundleResult {
    pub bundle_id: String,
    pub persistence_receipt_id: String,
    pub resource_snapshot_id: String,
    pub module_count: usize,
    pub event_log_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskStateChangedResult {
    pub task_id: String,
    pub status: String,
    pub event_id: String,
    pub sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskBoardResult {
    pub task_count: usize,
    pub latest_sequence: u64,
    pub tasks: Vec<TaskRecordResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TaskRecordResult {
    pub task_id: String,
    pub team_id: String,
    pub status: String,
    pub target_kind: String,
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
pub struct TmuxLoopbackResult {
    pub requested_count: usize,
    pub observed_count: usize,
    pub cleaned_handle_count: usize,
    pub cleanup_status: String,
    pub observations: Vec<TmuxLoopbackObservationResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TmuxLoopbackObservationResult {
    pub logical_agent_id: String,
    pub input_delivered: bool,
    pub output_observed: bool,
    pub ready_observed: bool,
    pub observed_text_bytes: usize,
}

pub fn config_result(normalized: NormalizedConfig) -> ConfigCheckResult {
    ConfigCheckResult {
        path: normalized.path,
        project_slug: normalized.project_slug,
        project_root: normalized.project_root,
        runtime_home: normalized.runtime_home,
        local_domain_id: normalized.local_domain_id,
        team_count: normalized.team_count,
        member_count: normalized.member_count,
        zterm_endpoint: normalized.zterm_endpoint,
        remote_domain_count: normalized.remote_domain_count,
    }
}

pub fn resolved_domain_result(resolved: ResolvedDomainTarget) -> ResolvedDomainTargetResult {
    let (target_kind, target_value) = target_kind_parts(resolved.target_kind);
    ResolvedDomainTargetResult {
        original_target: resolved.original_target,
        target_kind,
        target_value,
        domain_id: resolved.domain_id,
        route_kind: route_kind_label(resolved.route_kind).to_owned(),
        endpoint_host: resolved.endpoint.host,
        endpoint_port: resolved.endpoint.port,
    }
}

pub fn domain_snapshot_result(snapshot: DomainRegistrySnapshot) -> DomainRegistrySnapshotResult {
    DomainRegistrySnapshotResult {
        local_domain_id: snapshot.local_domain_id,
        aliases: snapshot.aliases,
        remote_domain_ids: snapshot.remote_domain_ids,
        endpoint_count: snapshot.endpoint_count,
        token_redaction_status: snapshot.token_redaction_status,
    }
}

pub fn daemon_check_result(
    normalized: ConfigCheckResult,
    snapshot: DomainRegistrySnapshot,
) -> DaemonCheckResult {
    DaemonCheckResult {
        project_slug: normalized.project_slug,
        runtime_home: normalized.runtime_home,
        local_domain_id: normalized.local_domain_id,
        routeable_endpoint_count: snapshot.endpoint_count,
        config_status: "valid".to_owned(),
        domain_registry_status: "routeable".to_owned(),
        daemon_process_status: "not_started_by_check".to_owned(),
        tmux_status: "not_touched_by_check".to_owned(),
        zterm_status: "not_touched_by_check".to_owned(),
    }
}

pub fn debug_bundle_result(bundle: DebugResp03Bundle, event_log: PathBuf) -> DebugBundleResult {
    DebugBundleResult {
        bundle_id: bundle.bundle_id,
        persistence_receipt_id: bundle.persistence_receipt_id,
        resource_snapshot_id: bundle.resource_snapshot_id,
        module_count: bundle.module_count,
        event_log_path: event_log.display().to_string(),
    }
}

pub fn task_changed_result(changed: TaskStateChanged) -> TaskStateChangedResult {
    TaskStateChangedResult {
        task_id: changed.task_id,
        status: changed.status.label().to_owned(),
        event_id: changed.event_id,
        sequence: changed.sequence,
    }
}

pub fn task_board_result(board: TaskBoard) -> TaskBoardResult {
    TaskBoardResult {
        task_count: board.task_count,
        latest_sequence: board.latest_sequence,
        tasks: board
            .tasks
            .into_iter()
            .map(|task| TaskRecordResult {
                task_id: task.task_id,
                team_id: task.team_id,
                status: task.status.label().to_owned(),
                target_kind: task.target_kind.label().to_owned(),
                target: task.target,
                title: task.title,
                body: task.body,
                priority: task.priority,
                blocked: task.blocked,
                created_by: task.created_by,
                latest_actor: task.latest_actor,
                latest_detail: task.latest_detail,
                latest_event_id: task.latest_event_id,
                latest_sequence: task.latest_sequence,
            })
            .collect(),
    }
}

pub fn tmux_loopback_result(report: TmuxLoopbackReport) -> TmuxLoopbackResult {
    TmuxLoopbackResult {
        requested_count: report.requested_count,
        observed_count: report.observed_count,
        cleaned_handle_count: report.cleaned_handle_count,
        cleanup_status: if report.all_observed() {
            "cleaned_exact_handles".to_owned()
        } else {
            "incomplete".to_owned()
        },
        observations: report
            .observations
            .into_iter()
            .map(|observation| TmuxLoopbackObservationResult {
                logical_agent_id: observation.logical_id,
                input_delivered: observation
                    .observed_text
                    .contains(&observation.input_marker),
                output_observed: observation
                    .observed_text
                    .contains(&observation.output_marker),
                ready_observed: observation
                    .observed_text
                    .contains("AGENTTEAM_LOOPBACK_READY:"),
                observed_text_bytes: observation.observed_text.len(),
            })
            .collect(),
    }
}

fn target_kind_parts(kind: DomainTargetKind) -> (String, String) {
    match kind {
        DomainTargetKind::Agent(value) => ("agent".to_owned(), value),
        DomainTargetKind::Role(value) => ("role".to_owned(), value),
        DomainTargetKind::Team(value) => ("team".to_owned(), value),
        DomainTargetKind::All => ("all".to_owned(), "all".to_owned()),
    }
}

fn route_kind_label(kind: DomainRouteKind) -> &'static str {
    match kind {
        DomainRouteKind::Local => "local",
        DomainRouteKind::Remote => "remote",
    }
}
