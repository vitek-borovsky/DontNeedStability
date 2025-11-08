use std::net::{UdpSocket, SocketAddr};
use std::io;

pub fn start_udp_server<F>(addr: SocketAddr, mut callback: F) -> io::Result<()>
where
    F: FnMut(&[u8], SocketAddr),
{
    let socket = UdpSocket::bind(addr)?;
    println!("UDP server listening on {}", addr);

    let mut buf = [0; 512]; // Standard DNS UDP packet size

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let received_data = &buf[..amt];
        callback(received_data, src);
    }
}
