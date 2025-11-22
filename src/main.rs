use std::net::SocketAddr;
use std::fs;
use serde::Deserialize;

use dont_need_stability::db::in_memory::InMemoryDatabase;
use dont_need_stability::app::App;
use dont_need_stability::server::ServerConfig;
use dont_need_stability::zone_parser::ZoneParser;

#[derive(Deserialize)]
struct Config {
    server: ServerConfig,
}

fn main() -> std::io::Result<()> {
    let config_content = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");
    let config: Config = toml::from_str(&config_content)
        .expect("Failed to parse config.toml");

    let socket: SocketAddr = format!("127.0.0.1:{}", config.server.port)
        .parse()
        .expect("Invalid address");

    let mut db = InMemoryDatabase::new();

    let zones_path = &config.server.zones_directory;
    if zones_path.is_dir() {
        for entry in fs::read_dir(zones_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "zone") {
                println!("Loading zone file: {:?}", path);
                let zone_content = fs::read_to_string(&path)?;
                let file_name = path.file_stem().and_then(|s| s.to_str()).unwrap_or("default.zone");
                let default_origin = format!("{}.", file_name.replace(".zone", ""));
                match ZoneParser::parse_zone_file(&zone_content, default_origin, 3600) {
                    Ok(zone) => {
                        db.add_zone(zone).expect("Failed to add zone to database");
                    },
                    Err(e) => eprintln!("Error parsing zone file {:?}: {}", path, e),
                }
            }
        }
    }

    let mut app: App = App::new(Box::new(db), socket);
    println!("Starting server on {}", socket);
    println!("Press Ctrl+C to stop");
    app.run();
    Ok(())
}
