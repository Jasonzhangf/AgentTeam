mod error;
mod loopback;
mod model;
mod process;
#[cfg(test)]
mod tests;

pub use error::{TmuxAdapterError, TmuxAdapterResult};
pub use loopback::run_tmux_loopback;
pub use model::{TmuxLoopbackInput, TmuxLoopbackObservation, TmuxLoopbackReport};

pub const FEATURE_ID: &str = "adapter.zterm_tmux";
