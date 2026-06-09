mod error;
mod model;
mod persist;
mod route;
#[cfg(test)]
mod tests;

pub use error::{CommCenterError, CommCenterResult};
pub use model::{
    CommBroadcastResult, CommBroadcastSendResult, CommMessageResult, CommMessageSendResult,
    CommReadyReportRequest, CommReadyReportResult, CommReadyReportSendResult, CommRouteRequest,
    CommRouteTarget, CommTaskBoardQueryRequest, CommTaskBoardQueryResult, CommTaskClaimRequest,
    CommTaskClaimResult, CommTeamBroadcastRequest,
};
pub use persist::persist_delivery_event;
pub use route::{
    route_broadcast, route_message, route_ready_report, route_task_board_query, route_task_claim,
    send_broadcast, send_message, send_ready_report, CommCenter,
};

pub const FEATURE_ID: &str = "comm.center";
