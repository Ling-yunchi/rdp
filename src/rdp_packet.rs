use std::mem;

/// # RDP Packet
/// header: 20 bytes
/// data: max 655
pub(crate) struct RdpPacket {
    pub(crate) header: RdpHeader,
    pub(crate) data: Vec<u8>,
}

/// # RDP Header
///
/// RDP Header is 1
///
/// | Field | Size | Description |
/// | --- | --- | --- |
/// | seq_num | 4 bytes | Sequence number |
/// | ack_num | 4 bytes | Acknowledgement number |
/// | flags | 1 byte | Flags |
/// | window_size | 2 bytes | Window size |
/// | checksum | 2 bytes | Checksum |
///
/// ## Flags
/// | Bit | Name | Description |
/// | --- | --- | --- |
/// | 0 | SYN | Synchronize |
/// | 1 | ACK | Acknowledge |
/// | 2 | FIN | Finish |
/// | 3 | RST | Reset |
pub(crate) struct RdpHeader {
    pub(crate) seq_num: u32,
    pub(crate) ack_num: u32,
    pub(crate) flags: u8,
    pub(crate) window_size: u16,
    pub(crate) checksum: u16,
}

impl RdpHeader {
    pub(crate) fn to_be_bytes(&self) -> [u8; mem::size_of::<Self>()] {
        let mut bytes = [0; mem::size_of::<Self>()];
        bytes[0..4].copy_from_slice(&self.seq_num.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.ack_num.to_be_bytes());
        bytes[8] = self.flags;
        bytes[9..11].copy_from_slice(&self.window_size.to_be_bytes());
        bytes[11..13].copy_from_slice(&self.checksum.to_be_bytes());
        bytes
    }
}

impl RdpPacket {
    pub(crate) fn new() -> RdpPacket {
        RdpPacket {
            header: RdpHeader {
                seq_num: 0,
                ack_num: 0,
                flags: 0,
                window_size: 0,
                checksum: 0,
            },
            data: Vec::new(),
        }
    }

    pub(crate) fn to_be_bytes(&self) -> [u8; mem::size_of::<Self>()] {
        let mut bytes = [0; mem::size_of::<Self>()];
        bytes[..mem::size_of::<RdpHeader>()].copy_from_slice(&self.header.to_be_bytes());
        bytes[mem::size_of::<RdpHeader>()..].copy_from_slice(&self.data);
        bytes
    }
}