mod error;
mod model;
mod route;
#[cfg(test)]
mod tests;

pub use error::{CommCenterError, CommCenterResult};
pub use model::{
    CommBroadcastResult, CommMessageResult, CommReadyReportRequest, CommReadyReportResult,
    CommRouteRequest, CommRouteTarget, CommTaskBoardQueryRequest, CommTaskBoardQueryResult,
    CommTaskClaimRequest, CommTaskClaimResult, CommTeamBroadcastRequest,
};
pub use route::{
    route_broadcast, route_message, route_ready_report, route_task_board_query, route_task_claim,
    CommCenter,
};

pub const FEATURE_ID: &str = "comm.center";
