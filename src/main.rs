use std::net::SocketAddr;

use crate::db::InMemoryDatabase;
use crate::app::app::App;

mod server;
mod app;
mod db;


fn main() -> std::io::Result<()> {
    let socket: SocketAddr = "127.0.0.1:5353".parse().expect("Invalid address");

    let db = InMemoryDatabase::new();
    let app: App = App::new(Box::new(db));
    println!("RUNNING ...");
    println!("Press Ctrl+C to stop");
    loop {}
}
