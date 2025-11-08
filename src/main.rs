use std::net::SocketAddr;
use std::thread;
use std::time::Duration;
use trust_dns_proto::op::Message;
use trust_dns_proto::serialize::binary::BinDecodable;

use crate::server::Server;

mod server;

fn example_callback(data: &[u8], src: SocketAddr) {
    println!("Processing packet from {:?}", src);
    match Message::from_bytes(data) {
        Ok(dns_message) => {
            println!("Decoded DNS Packet: {:?}", dns_message);
            // In a real application, you would process the DNS query and send a response
        },
        Err(e) => {
            eprintln!("Error decoding DNS packet: {}", e);
        }
    }
}

fn main() -> std::io::Result<()> {
    let socket: SocketAddr = "127.0.0.1:5353".parse().expect("Invalid address");
    let mut s = Server::new(socket, example_callback);
    s.run();

    println!("Server running...");
    thread::sleep(Duration::from_secs(10));
    s.stop();
    println!("Server stopped...");

    Ok(())
}
