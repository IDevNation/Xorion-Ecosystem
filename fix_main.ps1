$code = @' 
use std::io::{BufRead, BufReader, Write}; 
use std::net::{TcpListener, TcpStream}; 
use std::thread; 
 
const SOCKET_ADDR: &str = "127.0.0.1:8080"; 
 
fn main() { 
    env_logger::init(); 
    log::info!("Xorion wallet scheme daemon starting..."); 
 
