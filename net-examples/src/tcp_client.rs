use clap::Parser;
use std::io::{stdout, Read, Write};
use std::net;

// See also: https://docs.rs/socket2/latest/socket2/struct.Socket.html for lower-level socket API

const MAX_LINE: usize = 1024;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address of server
    #[arg(short, long, default_value = "localhost")]
    address: String,

    /// Port to connect to server on
    #[arg(short, long, default_value_t = 12000)]
    port: u16,
}

fn main() {
    // Process commandline arguments
    let args = Args::parse();

    let host = args.address;
    let port = args.port;

    // Create TCP socket and connect to specified server
    let mut socket = net::TcpStream::connect(format!("{}:{}", host, port)).unwrap();
    println!("Connected to {}", socket.peer_addr().unwrap());

    // Read from keyboard
    print!("Enter your message: ");
    stdout().flush().unwrap(); // force print to screen
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    // Send to server
    let bytes = socket.write(input.as_bytes()).unwrap();
    println!("Sent {bytes:?} bytes to server: {}", input);

    // Receive reply from server
    let mut buf: [u8; MAX_LINE] = [0; MAX_LINE];
    let bytes = socket.read(&mut buf).unwrap();

    // Print out received message
    println!(
        "Received {bytes:?} bytes from {} {}",
        socket.peer_addr().unwrap(),
        std::str::from_utf8(&buf).unwrap()
    );

    // Connection is automatically closed when stream goes out of scope
}
