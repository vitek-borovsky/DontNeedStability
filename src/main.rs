mod server;

fn main() -> std::io::Result<()> {
    server::start_udp_server()
}
