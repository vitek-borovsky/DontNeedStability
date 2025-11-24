use std::net::SocketAddr;
use std::sync::mpsc::{self, Receiver, Sender};
use trust_dns_proto::op::{Message, MessageType, OpCode, ResponseCode};
use trust_dns_proto::rr::{Name, RData, Record};
use trust_dns_proto::serialize::binary::{BinDecodable, BinDecoder, BinEncodable};

use crate::db::Database;
use crate::server::Server;

type D = dyn Database + Send + 'static;
type Payload = (Message, SocketAddr);

pub struct App {
    database: Box<D>,
    server: Server,
    tx: Sender<Payload>,
    rx: Receiver<Payload>,
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

    pub fn accept_udp_packet(
        tx: &Sender<Payload>,
        data: &[u8],
        src: SocketAddr,
        _socket: &std::net::UdpSocket,
    ) {
        match App::parse_dns_packet(data) {
            Err(e) => {
                eprintln!("Failed to parse packet {:?}", e);
            }
            Ok(msg) => {
                let _ = tx.send((msg, src));
            }
        };
    }

    fn process_message(&self) {
        match self.rx.try_recv() {
            Ok((msg, src)) => self.handle_message(msg, src, self.server.socket()),
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                // no message yet — do something else or sleep briefly
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {} // #FIXME
        }
    }

    fn handle_message(&self, msg: Message, src: SocketAddr, socket: &std::net::UdpSocket) {
        if msg.queries().is_empty() {
            return;
        }
        let question = &msg.queries()[0];
        let fqdn = question.name().to_string();
        let record_type = App::convert_record_type(question.query_type());
        let result = self.database.lookup_resource_record(&fqdn, record_type);

        let mut response = Message::new();
        response
            .set_id(msg.id())
            .set_message_type(MessageType::Response)
            .set_op_code(OpCode::Query)
            .set_authoritative(true)
            .add_query(question.clone());

        match result {
            Ok(Some(record_data)) => {
                let mut record = Record::new();
                record.set_name(Name::from_utf8(fqdn).unwrap());
                record.set_ttl(3600);
                record.set_rr_type(question.query_type());
                let rdata = App::convert_record_data(record_data);
                record.set_data(Some(rdata));
                response.add_answer(record);
            }
            Ok(None) => {
                response.set_response_code(ResponseCode::NXDomain);
            }
            Err(_) => {
                response.set_response_code(ResponseCode::ServFail);
            }
        }

        let mut response_buffer = Vec::new();
        let mut encoder = trust_dns_proto::serialize::binary::BinEncoder::new(&mut response_buffer);
        response.emit(&mut encoder).unwrap();
        let _ = socket.send_to(&response_buffer, src);
    }

    fn convert_record_type(
        record_type: trust_dns_proto::rr::RecordType,
    ) -> crate::db::RecordType {
        match record_type {
            // FIXME: Make everything use trust_dns_proto::rr::RecordType instead of my own crate::db::RecordType
            trust_dns_proto::rr::RecordType::A => crate::db::RecordType::A,
            trust_dns_proto::rr::RecordType::AAAA => crate::db::RecordType::AAAA,
            trust_dns_proto::rr::RecordType::CNAME => crate::db::RecordType::CNAME,
            trust_dns_proto::rr::RecordType::MX => crate::db::RecordType::MX,
            trust_dns_proto::rr::RecordType::NS => crate::db::RecordType::NS,
            trust_dns_proto::rr::RecordType::PTR => crate::db::RecordType::PTR,
            trust_dns_proto::rr::RecordType::SOA => crate::db::RecordType::SOA,
            trust_dns_proto::rr::RecordType::SRV => crate::db::RecordType::SRV,
            trust_dns_proto::rr::RecordType::TXT => crate::db::RecordType::TXT,
            _ => unimplemented!(),
        }
    }

    fn convert_record_data(record_data: &crate::db::RecordData) -> RData {
        match record_data {
            crate::db::RecordData::A(addr) => RData::A(*addr),
            crate::db::RecordData::AAAA(addr) => RData::AAAA(*addr),
            crate::db::RecordData::CNAME(name) => RData::CNAME(Name::from_utf8(name).unwrap()),
            crate::db::RecordData::MX { exchange, preference } => {
                RData::MX(trust_dns_proto::rr::rdata::MX::new(
                    *preference,
                    Name::from_utf8(exchange).unwrap(),
                ))
            }
            crate::db::RecordData::NS(name) => RData::NS(Name::from_utf8(name).unwrap()),
            crate::db::RecordData::PTR(name) => RData::PTR(Name::from_utf8(name).unwrap()),
            crate::db::RecordData::SOA {
                mname,
                rname,
                serial,
                refresh,
                retry,
                expire,
                minimum,
            } => RData::SOA(trust_dns_proto::rr::rdata::SOA::new(
                Name::from_utf8(mname).unwrap(),
                Name::from_utf8(rname).unwrap(),
                *serial,
                *refresh as i32,
                *retry as i32,
                *expire as i32,
                *minimum,
            )),
            crate::db::RecordData::SRV {
                priority,
                weight,
                port,
                target,
            } => RData::SRV(trust_dns_proto::rr::rdata::SRV::new(
                *priority,
                *weight,
                *port,
                Name::from_utf8(target).unwrap(),
            )),
            crate::db::RecordData::TXT(txt) => {
                RData::TXT(trust_dns_proto::rr::rdata::TXT::new(vec![txt.clone()]))
            }
        }
    }
}
