use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, Sender};
use trust_dns_proto::op::Message;
use trust_dns_proto::serialize::binary::{BinDecodable, BinDecoder};

use crate::db::Database;
use crate::server::Server;

type D = dyn Database + Send + 'static;
type Payload = (Message, SocketAddr);

pub struct App {
    database: Box<D>,
    server: Server,
    tx: Sender<Payload>,
    rx: Receiver<Payload>,
    // TODO: implement worker that will read channel
    // with rx and make requests to database, interpret data and
    // eventually responds to the socket
}



impl App {
    pub fn new(database: Box<D>, ) -> Self {
        let socket: SocketAddr = "127.0.0.1:5353".parse().expect("Invalid address");
        let (tx, rx) = mpsc::channel();
        let server = Server::new(socket);
        let mut a: App = App {
            database,
            server,
            tx,
            rx,
        };

        let tx_cloned = a.tx.clone();
        let callback = move |data: &[u8], socket: SocketAddr| {
            App::accept_udp_packet(&tx_cloned, data, socket);
        };
        a.server.register_callback(Box::new(callback));
        a
    }

    pub fn run(&mut self) {
        self.server.run();
    }


    fn parse_dns_packet(buf: &[u8]) -> Result<Message, Box<dyn std::error::Error>> {
        let mut decoder = BinDecoder::new(buf);
        let msg: Message = Message::read(&mut decoder)?;
        Ok(msg)
    }

    pub fn accept_udp_packet(tx: &Sender<Payload>, data: &[u8], src: SocketAddr) {
        match App::parse_dns_packet(data) {
            Err(e) => { eprintln!("Failed to parse packet {:?}", e); },
            Ok(msg) => { let _ = tx.send((msg, src)); }
        };
    }
}
