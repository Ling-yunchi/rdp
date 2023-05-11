pub mod rdp;
mod rdp_packet;
mod rdp_option;
mod test_udp;
mod rdp_utils;

#[cfg(test)]
mod tests {
    use crate::rdp::RdpStream;

    #[test]
    fn test_rdp() {
    }
}
