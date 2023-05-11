#[cfg(test)]
mod tests {
    use std::net::{TcpListener, TcpStream, UdpSocket};
    use std::thread;

    #[test]
    fn test_main() {
        let tcp_stream = TcpStream::connect("localhost:8080").unwrap();
    }

    #[test]
    fn test_udp_listen() {
        let server_socket = UdpSocket::bind("0.0.0.0:8080").unwrap();
        let client_socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        let handler = thread::spawn(move || {
            let mut i = 0;
            loop {
                if i >= 2 {
                    break;
                }
                let mut buf = [0; 10];
                let (amt, src) = server_socket.recv_from(&mut buf).unwrap();
                println!("server recv {:?} from {:?}", &buf[..amt], src);
                server_socket.send_to(&buf[..amt], &src).unwrap();
                i += 1;
            }
        });

        client_socket.connect("localhost:8080").unwrap();
        let buf = [1; 10];
        client_socket.send(&buf).unwrap();
        let mut buf = [0; 10];
        let (amt, addr) = client_socket.recv_from(&mut buf).unwrap();
        println!("client recv {:?} from {:?}", &buf[..amt], addr);

        client_socket.send(&buf[..amt]).unwrap();
        let mut buf = [0; 10];
        let (amt, addr) = client_socket.recv_from(&mut buf).unwrap();
        println!("client recv {:?} from {:?}", &buf[..amt], addr);

        handler.join().unwrap();
    }

    #[test]
    fn test_tcp() {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to address");

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New client connected: {}", stream.peer_addr().unwrap());
                }
                Err(e) => {
                    println!("Error accepting client: {}", e);
                }
            }
        }
    }
}