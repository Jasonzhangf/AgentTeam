use serde::Serialize;

use crate::pipeline::PipelineNodeName;

pub const FEATURE_ID: &str = "adapter.zterm_tmux";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminalReq01AdapterCommand {
    pub runtime_scope: String,
    pub session_count: usize,
}

impl TerminalReq01AdapterCommand {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Terminal", "Req", 1, "AdapterCommand");

    pub fn loopback(runtime_scope: String, session_count: usize) -> Self {
        Self {
            runtime_scope,
            session_count,
        }
    }

    pub fn prepare_transport(self, session_prefix: String) -> TerminalReq02TransportRequest {
        TerminalReq02TransportRequest {
            runtime_scope: self.runtime_scope,
            session_count: self.session_count,
            session_prefix,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminalReq02TransportRequest {
    pub runtime_scope: String,
    pub session_count: usize,
    pub session_prefix: String,
}

impl TerminalReq02TransportRequest {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Terminal", "Req", 2, "TransportRequest");

    pub fn transport_event(
        &self,
        logical_id: String,
        event_kind: String,
        marker: String,
    ) -> TerminalResp03TransportEvent {
        TerminalResp03TransportEvent {
            runtime_scope: self.runtime_scope.clone(),
            logical_id,
            event_kind,
            marker,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminalResp03TransportEvent {
    pub runtime_scope: String,
    pub logical_id: String,
    pub event_kind: String,
    pub marker: String,
}

impl TerminalResp03TransportEvent {
    pub const NODE: PipelineNodeName =
        PipelineNodeName::new("Terminal", "Resp", 3, "TransportEvent");

    pub fn observe(
        self,
        input_marker: String,
        output_marker: String,
        observed_text: String,
    ) -> TerminalResp04Observation {
        TerminalResp04Observation {
            runtime_scope: self.runtime_scope,
            logical_id: self.logical_id,
            input_marker,
            output_marker,
            observed_text,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TerminalResp04Observation {
    pub runtime_scope: String,
    pub logical_id: String,
    pub input_marker: String,
    pub output_marker: String,
    pub observed_text: String,
}

impl TerminalResp04Observation {
    pub const NODE: PipelineNodeName = PipelineNodeName::new("Terminal", "Resp", 4, "Observation");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_chain_uses_adjacent_nodes() {
        let command = TerminalReq01AdapterCommand::loopback("agentteam".to_owned(), 2);
        let request = command.prepare_transport("TA-agentteam-loopback".to_owned());
        let event = request.transport_event(
            "agent-01".to_owned(),
            "stdout".to_owned(),
            "ready".to_owned(),
        );
        let observation = event.observe(
            "input-01".to_owned(),
            "output-01".to_owned(),
            "output-01".to_owned(),
        );

        assert_eq!(TerminalReq01AdapterCommand::NODE.number, 1);
        assert_eq!(TerminalReq02TransportRequest::NODE.number, 2);
        assert_eq!(TerminalResp03TransportEvent::NODE.number, 3);
        assert_eq!(TerminalResp04Observation::NODE.number, 4);
        assert_eq!(observation.logical_id, "agent-01");
    }

    #[test]
    fn terminal_feature_id_is_stable() {
        assert_eq!(FEATURE_ID, "adapter.zterm_tmux");
    }
}
