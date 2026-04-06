use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const SOCKET_ADDR: &str = "127.0.0.1:8080";

fn main() {
    env_logger::init();
    log::info!("Xorion wallet scheme daemon starting...");

    let listener = TcpListener::bind(SOCKET_ADDR).unwrap_or_else(|e| {
        eprintln!("failed to bind {}: {}", SOCKET_ADDR, e);
        std::process::exit(1);
    });

    log::info!("listening on {}", SOCKET_ADDR);
    println!("Xorion wallet daemon listening on {}", SOCKET_ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => log::error!("accept error: {}", e),
        }
    }
}

fn handle_client(stream: TcpStream) {
    let reader: BufReader<&TcpStream> = BufReader::new(&stream);
    let mut writer = stream.try_clone().expect("failed to clone stream");

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        if line.trim().is_empty() {
            continue;
        }

        let response = match serde_json::from_str::<serde_json::Value>(&line) {
            Ok(req) => handle_request(req),
            Err(e) => format!(r#"{{"error":"{}"}}"#, e),
        };

        if let Err(e) = writeln!(writer, "{}", response) {
            log::error!("failed to write response: {}", e);
            break;
        }
    }
}

fn handle_request(req: serde_json::Value) -> String {
    if let Some(cmd) = req.get("cmd").and_then(|v| v.as_str()) {
        match cmd {
            "status" => r#"{\"status\":\"ok\",\"message\":\"Xorion wallet is running\"}"#.to_string(),
            _ => r#"{\"error\":\"unknown command\"}"#.to_string(),
        }
    } else {
        r#"{\"error\":\"invalid request format\"}"#.to_string()
    }
}
