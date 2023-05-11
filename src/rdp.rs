use std::collections::HashMap;
use std::error::Error;
use std::net::{ToSocketAddrs, UdpSocket};
use std::result;
use crate::rdp_option::RdpOption;
use crate::rdp_packet::RdpPacket;

pub type RdpError = Box<dyn Error>;

pub type Result<T> = result::Result<T, RdpError>;

/// RDP Stream
/// a connection-oriented protocol based on UDP
pub struct RdpStream {
    send_socket: UdpSocket,
    recv_socket: UdpSocket,
    option: RdpOption,
    buffer: Vec<u8>,
}

impl RdpStream {
    /// Connect to the remote host.
    /// RdpStream is a connection-oriented protocol, so you must call this method before sending data.
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<RdpStream> {
        let send_socket = UdpSocket::bind(addr)?;
        let recv_socket = UdpSocket::bind("0.0.0.0:0")?;
        // let local_addr = recv_socket.local_addr()?;
        // println!("local_addr: {}", local_addr);
        // // send SYN
        // let mut packet = RdpPacket::new();
        // packet.set_syn();
        // packet.set_seq(0);
        // packet.set_ack(0);
        // packet.set_mss(1460);
        // packet.set_window_size(65535);
        // packet.set_payload(&[]);
        // send_socket.send_to(packet.as_bytes(), addr)?;
        // // recv SYN+ACK
        // let mut buffer = [0; 1500];
        // let (size, _) = recv_socket.recv_from(&mut buffer)?;
        // let packet = RdpPacket::from_bytes(&buffer[..size])?;
        // if !packet.is_syn() || !packet.is_ack() {
        //     return Err("SYN+ACK expected".into());
        // }
        // // send ACK
        // let mut packet = RdpPacket::new();
        // packet.set_ack(1);
        // packet.set_seq(1);
        // packet.set_mss(1460);
        // packet.set_window_size(65535);
        // packet.set_payload(&[]);
        // send_socket.send_to(packet.as_bytes(), addr)?;

        let option = RdpOption::default();
        let buffer = Vec::new();
        Ok(RdpStream {
            send_socket,
            recv_socket,
            option,
            buffer,
        })
    }
}

pub struct RdpListener {
    listen_socket: UdpSocket,
    connect_socket: HashMap<(), UdpSocket>,
}

impl RdpListener {}