use std::net::UdpSocket;
use std::io;
use trust_dns_proto::op::Message;
use trust_dns_proto::serialize::binary::BinDecodable;

pub fn start_udp_server() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:5353")?;
    println!("UDP server listening on 127.0.0.1:5353");

    let mut buf = [0; 512]; // Standard DNS UDP packet size

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let received_data = &buf[..amt];

        println!("Received {} bytes from {}", amt, src);

        match Message::from_bytes(received_data) {
            Ok(dns_message) => {
                println!("Decoded DNS Packet: {:?}", dns_message);
                // In a real application, you would process the DNS query and send a response
            },
            Err(e) => {
                eprintln!("Error decoding DNS packet: {}", e);
            }
        }
    }
}

