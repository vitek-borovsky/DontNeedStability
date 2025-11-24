use dont_need_stability::server::{Server};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use rand::{Rng, RngCore, thread_rng};
use std::thread;
use std::time::Duration;

fn gen_buffer_size_512() -> Vec<u8> {
    let mut rng = thread_rng();
    let len = rng.gen_range(1..=512);
    let mut bytes = vec![0u8; len];
    rng.fill_bytes(&mut bytes);
    bytes
}

#[test]
fn test_udp_packet_capture() -> std::io::Result<()> {
    let test_packet: Vec<u8> = gen_buffer_size_512();
    let clonned_packet: Vec<u8> = test_packet.clone();
    let server_socket_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 5353);
    let client_socket_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 5354);
    let client_socket = UdpSocket::bind(client_socket_addr)?;

    let callback = move |data: &[u8], _src: SocketAddr, _: &UdpSocket| {
        assert_eq!(data, &clonned_packet[..]);
    };

    let mut serv = Server::new(server_socket_addr);
    serv.register_callback(Box::new(callback));
    serv.run();

    thread::sleep(Duration::from_secs(1));

    client_socket.send_to(&test_packet, server_socket_addr)?;

    // Give the server time to receive
    thread::sleep(Duration::from_secs(1));
    serv.stop();

    Ok(())
}
