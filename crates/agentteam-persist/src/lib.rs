mod append;
mod error;
mod materialize;
mod model;
mod replay;
#[cfg(test)]
mod tests;

pub use append::append_event_log;
pub use error::{PersistenceError, PersistenceResult};
pub use materialize::materialize_event_log;
pub use model::{PersistedEventDraft, PersistedEventRecord, ReplayedEventLog};
pub use replay::replay_event_log;

pub const FEATURE_ID: &str = "persist.event_log";
