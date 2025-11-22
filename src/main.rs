use std::net::SocketAddr;

use dontNeedStability::db::in_memory::InMemoryDatabase;
use dontNeedStability::app::App;

fn main() -> std::io::Result<()> {
    let socket: SocketAddr = "127.0.0.1:5353".parse().expect("Invalid address");

    let db = InMemoryDatabase::new();
    let mut app: App = App::new(Box::new(db), socket);
    println!("Starting server ...");
    println!("Press Ctrl+C to stop");
    app.run();
    Ok(())
}
