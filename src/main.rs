use std::net::SocketAddr;
use trust_dns_proto::op::Message;
use trust_dns_proto::serialize::binary::BinDecodable;

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
    let addr: SocketAddr = "127.0.0.1:5353".parse().expect("Invalid address");
    server::start_udp_server(addr, example_callback)?;

    Ok(())
}
