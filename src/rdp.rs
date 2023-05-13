use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs, UdpSocket};
use std::result;
use log::debug;
use crate::rdp_option::RdpOption;
use crate::rdp_packet::{RdpHeader, RdpPacket};
use crate::rdp_utils::get_random_seq;

pub type RdpError = Box<dyn Error>;

pub type Result<T> = result::Result<T, RdpError>;

/// RDP Stream
/// a connection-oriented protocol based on UDP
pub struct RdpStream {
    socket: UdpSocket,
    option: RdpOption,
    buffer: Vec<u8>,
}

impl RdpStream {
    /// Connect to the remote host.
    /// RdpStream is a connection-oriented protocol, so you must call this method before sending data.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<RdpStream> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(addr)?;
        let option = RdpOption::default();
        let buffer = Vec::new();
        let rdp_stream = RdpStream {
            socket,
            option,
            buffer,
        };
        rdp_stream.handshake()?;
        Ok(rdp_stream)
    }

    fn handshake(&self) -> Result<()> {
        // send syn
        debug!("send SYN packet to {}", self.socket.peer_addr()?);
        let mut packet = RdpPacket {
            header: RdpHeader {
                seq_num: get_random_seq(),
                ack_num: 0,
                flags: 0b00000001,
                window_size: 0,
                checksum: 0,
            },
            data: Vec::new(),
        };
        self.socket.send(&packet.to_be_bytes())?;

        // receive syn-ack
        debug!("receive SYN-ACK packet from {}", self.socket.peer_addr()?);
        let mut buf = [0; 1024];
        self.socket.recv(&mut buf)?;
        let syn_ack_packet = RdpPacket::from_be_bytes(&buf)?;
        if syn_ack_packet.header.flags != 0b00000011 || syn_ack_packet.header.ack_num != packet.header.seq_num + 1 {
            return Err("three-way handshake failed".into());
        }

        // send ack
        debug!("send ACK packet to {}", self.socket.peer_addr()?);
        let mut packet = RdpPacket::new();
        packet.header.flags = 0b00000010;
        packet.header.ack_num = syn_ack_packet.header.seq_num + 1;
        packet.data = Vec::new();
        self.socket.send(&packet.to_be_bytes())?;

        debug!("three-way handshake success");
        Ok(())
    }
}

pub struct RdpListener {
    listen_socket: UdpSocket,
    connect_socket: HashSet<SocketAddr>,
    connecting_socket: HashMap<SocketAddr, u32>,
}

pub struct Incoming<'a> {
    listener: &'a mut RdpListener,
}

impl RdpListener {
    pub fn bind<A: ToSocketAddrs>(addr: A) -> Result<RdpListener> {
        let listen_socket = UdpSocket::bind(addr)?;
        let connect_socket = HashSet::new();
        let rdp_listener = RdpListener {
            listen_socket,
            connect_socket,
            connecting_socket: HashMap::new(),
        };
        Ok(rdp_listener)
    }

    pub fn accept(&mut self) -> Result<RdpStream> {
        let mut buf = [0; 1480];
        loop {
            let (num, addr) = self.listen_socket.recv_from(&mut buf)?;
            let packet = RdpPacket::from_be_bytes(&buf[..num])?;
            if let Some(stream) = self.handle_packet(addr, packet)? {
                return Ok(stream);
            }
        }
    }

    fn handle_packet(&mut self, addr: SocketAddr, packet: RdpPacket) -> Result<Option<RdpStream>> {
        if packet.header.syn() {
            // SYN packet, send SYN-ACK
            debug!("receive SYN packet from {}", addr);
            let mut syn_ack_packet = RdpPacket::new();
            syn_ack_packet.header.seq_num = get_random_seq();
            syn_ack_packet.header.ack_num = packet.header.seq_num + 1;
            syn_ack_packet.header.flags = 0b00000011;
            self.connecting_socket.insert(addr, syn_ack_packet.header.seq_num);
            self.listen_socket.send_to(&syn_ack_packet.to_be_bytes(), addr)?;
        } else if packet.header.ack() {
            // ACK packet
            debug!("receive ACK packet from {}", addr);
            if !self.connect_socket.contains(&addr) {
                // three-way handshake
                if let Some(seq_num) = self.connecting_socket.get(&addr) {
                    // check ack_num is seq_num + 1
                    if packet.header.ack_num == seq_num + 1 {
                        self.connect_socket.insert(addr);
                        self.connecting_socket.remove(&addr);
                        return Ok(Some(RdpStream {
                            socket: self.listen_socket.try_clone()?,
                            option: RdpOption::default(),
                            buffer: Vec::new(),
                        }));
                    }
                    // else do nothing
                } else {
                    // receive data ack
                }
            }
        } else if packet.header.fin() {
            // FIN packet
            debug!("receive FIN packet from {}", addr);
            self.connect_socket.remove(&addr);
            // send ACK
            debug!("send ACK packet to {}", addr);
            let mut ack_packet = RdpPacket::new();
            ack_packet.header.seq_num = get_random_seq();
            ack_packet.header.ack_num = packet.header.seq_num + 1;
            ack_packet.header.flags = 0b00000100;
            self.listen_socket.send_to(&ack_packet.to_be_bytes(), addr)?;

            // send FIN
            debug!("send FIN packet to {}", addr);
            let mut fin_packet = RdpPacket::new();
            fin_packet.header.seq_num = get_random_seq();
            fin_packet.header.ack_num = 0;
            fin_packet.header.flags = 0b00000101;
            self.listen_socket.send_to(&fin_packet.to_be_bytes(), addr)?;
        }
        Ok(None)
    }

    pub fn incoming(&mut self) -> Incoming {
        Incoming {
            listener: self,
        }
    }
}

impl<'a> Iterator for Incoming<'a> {
    type Item = Result<RdpStream>;
    fn next(&mut self) -> Option<Result<RdpStream>> {
        Some(self.listener.accept())
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use log::debug;
    use crate::logger::SimpleLogger;
    use crate::rdp::{RdpListener, RdpStream};

    static LOGGER: SimpleLogger = SimpleLogger;

    #[test]
    fn test() {
        log::set_logger(&LOGGER).map(|()| log::set_max_level(log::LevelFilter::Debug)).unwrap();
        let mut rdp_listener = RdpListener::bind("0.0.0.0:8000").unwrap();
        thread::spawn(move || { let mut rdp_stream = rdp_listener.accept().unwrap(); });
        let mut rdp_stream = RdpStream::connect("127.0.0.1:8000").unwrap();
    }
}
