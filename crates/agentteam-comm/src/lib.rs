mod error;
mod model;
mod route;
#[cfg(test)]
mod tests;

pub use error::{CommCenterError, CommCenterResult};
pub use model::{
    CommBroadcastResult, CommMessageResult, CommRouteRequest, CommRouteTarget,
    CommTeamBroadcastRequest,
};
pub use route::{route_broadcast, route_message, CommCenter};

pub const FEATURE_ID: &str = "comm.center";
