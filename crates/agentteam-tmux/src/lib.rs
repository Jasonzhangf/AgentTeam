mod control;
mod error;
mod launch;
mod loopback;
mod model;
mod process;
#[cfg(test)]
mod tests;

pub use control::{capture_session, interrupt_session, send_input, session_exists, stop_session};
pub use error::{TmuxAdapterError, TmuxAdapterResult};
pub use launch::launch_managed_session;
pub use loopback::run_tmux_loopback;
pub use model::{
    TmuxLaunchInput, TmuxLaunchReport, TmuxLoopbackInput, TmuxLoopbackObservation,
    TmuxLoopbackReport,
};

pub const FEATURE_ID: &str = "adapter.zterm_tmux";
