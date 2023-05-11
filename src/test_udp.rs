#[cfg(test)]
mod tests {
    use std::net::{TcpStream, UdpSocket};

    #[test]
    fn test_main() {
        let tcp_stream = TcpStream::connect("localhost:8080").unwrap();
    }

    #[test]
    fn test_udp_listen() {
        let udp_socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        let local_addr = udp_socket.local_addr().unwrap();
        println!("local_addr: {}", local_addr);
    }
}