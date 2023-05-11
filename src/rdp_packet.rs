use std::mem;
use crate::rdp;

/// # RDP Header
///
/// RDP Header is 13 bytes.
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

const HEADER_SIZE: usize = 13;

/// # RDP Packet
/// header: 13 bytes
/// data: 0 ~ 1467 bytes
pub(crate) struct RdpPacket {
    pub(crate) header: RdpHeader,
    pub(crate) data: Vec<u8>,
}

impl RdpHeader {
    pub(crate) fn syn(&self) -> bool {
        0b00000001 & self.flags != 0
    }
    pub(crate) fn ack(&self) -> bool {
        0b00000010 & self.flags != 0
    }
    pub(crate) fn fin(&self) -> bool {
        0b00000100 & self.flags != 0
    }
    pub(crate) fn rst(&self) -> bool {
        0b00001000 & self.flags != 0
    }

    pub(crate) fn to_be_bytes(&self) -> [u8; HEADER_SIZE] {
        let mut bytes = [0; HEADER_SIZE];
        bytes[0..4].copy_from_slice(&self.seq_num.to_be_bytes());
        bytes[4..8].copy_from_slice(&self.ack_num.to_be_bytes());
        bytes[8] = self.flags;
        bytes[9..11].copy_from_slice(&self.window_size.to_be_bytes());
        bytes[11..13].copy_from_slice(&self.checksum.to_be_bytes());
        bytes
    }

    pub(crate) fn from_be_bytes(bytes: &[u8]) -> rdp::Result<Self> {
        if bytes.len() != HEADER_SIZE {
            return Err("invalid RDP header".into());
        }
        let mut header = RdpHeader {
            seq_num: 0,
            ack_num: 0,
            flags: 0,
            window_size: 0,
            checksum: 0,
        };
        header.seq_num = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        header.ack_num = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
        header.flags = bytes[8];
        header.window_size = u16::from_be_bytes(bytes[9..11].try_into().unwrap());
        header.checksum = u16::from_be_bytes(bytes[11..13].try_into().unwrap());
        Ok(header)
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

    pub(crate) fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0; HEADER_SIZE + self.data.len()];
        bytes[..HEADER_SIZE].copy_from_slice(&self.header.to_be_bytes());
        bytes[HEADER_SIZE..].copy_from_slice(&self.data);
        bytes
    }

    pub(crate) fn from_be_bytes(bytes: &[u8]) -> rdp::Result<Self> {
        let header = RdpHeader::from_be_bytes(&bytes[..HEADER_SIZE])?;
        Ok(RdpPacket {
            header,
            data: bytes[HEADER_SIZE..].to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::rdp_packet::RdpHeader;
    use crate::rdp_utils::get_random_seq;

    #[test]
    fn test_rdp_header() {
        let header = RdpHeader {
            seq_num: 0x114514,
            ack_num: 0x1919810,
            flags: 0b00000001,
            window_size: 0,
            checksum: 0,
        };
        let res: [u8; 13] = [0, 17, 69, 20, 1, 145, 152, 16, 1, 0, 0, 0, 0];
        let bytes = header.to_be_bytes();

        let header1 = RdpHeader::from_be_bytes(&res).unwrap();

        assert_eq!(bytes, res);
        assert_eq!(header1.seq_num, 0x114514);
        assert_eq!(header1.ack_num, 0x1919810);
        assert_eq!(header1.flags, 0b00000001);
        assert_eq!(header1.window_size, 0);
        assert_eq!(header1.checksum, 0);
    }

    #[test]
    fn test_rdp_packet() {
        let mut packet = crate::rdp_packet::RdpPacket::new();
        packet.header.seq_num = get_random_seq();
        packet.header.ack_num = get_random_seq();
        packet.header.flags = 0b00000001;
        packet.header.window_size = 0;
        packet.header.checksum = 0;
        packet.data = vec![0; 1460];
        let bytes = packet.to_be_bytes();
        let packet1 = crate::rdp_packet::RdpPacket::from_be_bytes(&bytes).unwrap();
        assert_eq!(packet.header.seq_num, packet1.header.seq_num);
        assert_eq!(packet.header.ack_num, packet1.header.ack_num);
        assert_eq!(packet.header.flags, packet1.header.flags);
        assert_eq!(packet.header.window_size, packet1.header.window_size);
        assert_eq!(packet.header.checksum, packet1.header.checksum);
        assert_eq!(packet.data, packet1.data);
    }
}