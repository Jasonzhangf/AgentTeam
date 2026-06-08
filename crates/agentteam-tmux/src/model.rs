#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TmuxLoopbackInput {
    pub runtime_home: String,
    pub session_count: usize,
}

impl TmuxLoopbackInput {
    pub fn new(runtime_home: String, session_count: usize) -> Self {
        Self {
            runtime_home,
            session_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TmuxLoopbackReport {
    pub requested_count: usize,
    pub observed_count: usize,
    pub cleaned_handle_count: usize,
    pub observations: Vec<TmuxLoopbackObservation>,
}

impl TmuxLoopbackReport {
    pub fn all_observed(&self) -> bool {
        self.requested_count == self.observed_count
            && self.requested_count == self.cleaned_handle_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TmuxLoopbackObservation {
    pub logical_id: String,
    pub input_marker: String,
    pub output_marker: String,
    pub observed_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ManagedTmuxSession {
    pub logical_id: String,
    pub session_name: String,
    pub ready_marker: String,
    pub input_marker: String,
    pub output_marker: String,
}
