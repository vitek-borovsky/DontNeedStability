use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, Sender};
use trust_dns_proto::op::Message;
use trust_dns_proto::serialize::binary::{BinDecodable, BinDecoder};

use crate::db::Database;
use crate::server::Server;

type D = dyn Database + Send + 'static;
type Payload = (Message, SocketAddr);

pub struct App {
    #[allow(dead_code)]
    database: Box<D>,
    server: Server,
    tx: Sender<Payload>,
    rx: Receiver<Payload>,
    // TODO: implement worker that will read channel
    // with rx and make requests to database, interpret data and
    // eventually responds to the socket
}



impl App {
    pub fn new(database: Box<D>, socket: SocketAddr) -> Self {
        let (tx, rx) = mpsc::channel();
        let server = Server::new(socket);
        let mut a: App = App {
            database,
            server,
            tx,
            rx,
        };

        let tx_cloned = a.tx.clone();
        let callback = move |data: &[u8], src: SocketAddr, socket: &std::net::UdpSocket| {
            App::accept_udp_packet(&tx_cloned, data, src, socket);
        };
        a.server.register_callback(Box::new(callback));
        a
    }

    pub fn run(&mut self) {
        self.server.run();
        loop {
            self.process_message();
        }
    }

    fn parse_dns_packet(buf: &[u8]) -> Result<Message, Box<dyn std::error::Error>> {
        let mut decoder = BinDecoder::new(buf);
        let msg: Message = Message::read(&mut decoder)?;
        Ok(msg)
    }

    pub fn accept_udp_packet(tx: &Sender<Payload>, data: &[u8], src: SocketAddr, _socket: &std::net::UdpSocket) {
        match App::parse_dns_packet(data) {
            Err(e) => { eprintln!("Failed to parse packet {:?}", e); },
            Ok(msg) => { let _ = tx.send((msg, src)); }
        };
    }

    fn process_message(&self) {
        match self.rx.try_recv() {
            Ok((msg, src)) => App::handle_message(msg, src, self.server.socket()),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // no message yet — do something else or sleep briefly
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {} // #FIXME
        }
    }

    fn handle_message(msg: Message, src: SocketAddr, socket: &std::net::UdpSocket) {
        println!("Processing message {:?} from {:?}", msg, src);
        // Placeholder: Echo back a simple response
        let response_data = b"DNS response placeholder";
        let _ = socket.send_to(response_data, src);
    }
}
