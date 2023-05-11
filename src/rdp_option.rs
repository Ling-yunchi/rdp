/// RDP option
/// - mss: Maximum Segment Size
pub(crate) struct RdpOption {
    pub(crate) mss: u16,
}

impl RdpOption {
    /// Create a new RDP option
    /// - mss: Maximum Segment Size
    pub(crate) fn new(mss: u16) -> RdpOption {
        RdpOption { mss }
    }

    /// default RDP option
    /// - mss: 1460
    pub(crate) fn default() -> RdpOption {
        RdpOption { mss: 1460 }
    }
}