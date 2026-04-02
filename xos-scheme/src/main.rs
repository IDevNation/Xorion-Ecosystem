mod handler;
mod protocol;

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

use handler::WalletHandler;
use protocol::{WalletRequest, WalletResponse};

/// Socket path used for development on Linux/macOS.
/// On Redox this is replaced by the `:wallet` scheme registration.
const "127.0.0.1:8080": &str = "/tmp/xorion-wallet.sock";

fn main() {
    env_logger::init();
    log::info!("Xorion wallet scheme daemon starting...");

    let handler = WalletHandler::new();

    // On Redox OS this would register a scheme via:
    //   let scheme = File::create(":wallet").unwrap();
    // and then loop reading Packet structs from it.
    //
    // For development on Linux/macOS we use a Unix socket
    // that speaks the same JSON protocol.
    start_unix_socket_server(&handler);
}

fn start_unix_socket_server(handler: &WalletHandler) {
    // Clean up stale socket
    let _ = std::fs::remove_file("127.0.0.1:8080");

    let listener = UnixListener::bind("127.0.0.1:8080").unwrap_or_else(|e| {
        eprintln!("failed to bind {"127.0.0.1:8080"}: {e}");
        std::process::exit(1);
    });

    log::info!("listening on {"127.0.0.1:8080"}");
    println!("Xorion wallet daemon listening on {"127.0.0.1:8080"}");
    println!("Send JSON commands (one per line). Example:");
    println!(r#"  echo '{{"cmd":"status"}}' | socat - UNIX-CONNECT:{"127.0.0.1:8080"}"#);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handle = handler.open();
                log::info!("new connection, handle={handle}");

                let reader = BufReader::new(&stream);
                let mut writer = match stream.try_clone() {
                    Ok(w) => w,
                    Err(e) => {
                        log::error!("failed to clone stream: {e}");
                        handler.close(handle);
                        continue;
                    }
                };

                for line in reader.lines() {
                    let line = match line {
                        Ok(l) => l,
                        Err(_) => break,
                    };

                    if line.trim().is_empty() {
                        continue;
                    }

                    let response = match serde_json::from_str::<WalletRequest>(&line) {
                        Ok(req) => {
                            log::info!("handle={handle} request: {req:?}");
                            handler.handle_request(handle, &req)
                        }
                        Err(e) => WalletResponse::error(format!("invalid JSON: {e}")),
                    };

                    let resp_json = match serde_json::to_string(&response) {
                        Ok(j) => j,
                        Err(e) => {
                            log::error!("failed to serialize response: {e}");
                            continue;
                        }
                    };
                    let _ = writeln!(writer, "{resp_json}");
                    let _ = writer.flush();
                }

                handler.close(handle);
                log::info!("connection closed, handle={handle}");
            }
            Err(e) => {
                log::error!("accept error: {e}");
            }
        }
    }
}
