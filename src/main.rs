use std::net::SocketAddr;
use std::fs;
use serde::Deserialize;

use dont_need_stability::db::in_memory::InMemoryDatabase;
use dont_need_stability::app::App;

#[derive(Deserialize)]
struct Config {
    server: ServerConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    port: u16,
}

fn main() -> std::io::Result<()> {
    let config_content = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_content)
        .expect("Failed to parse config.toml");

    let socket: SocketAddr = format!("127.0.0.1:{}", config.server.port)
        .parse()
        .expect("Invalid address");

    let db = InMemoryDatabase::new();
    let mut app: App = App::new(Box::new(db), socket);
    println!("Starting server on {}", socket);
    println!("Press Ctrl+C to stop");
    app.run();
    Ok(())
}
