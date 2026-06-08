#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineNodeName {
    pub domain: &'static str,
    pub direction: &'static str,
    pub number: u8,
    pub node: &'static str,
}

impl PipelineNodeName {
    pub const fn new(
        domain: &'static str,
        direction: &'static str,
        number: u8,
        node: &'static str,
    ) -> Self {
        Self {
            domain,
            direction,
            number,
            node,
        }
    }
}
