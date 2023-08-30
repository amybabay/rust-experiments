use clap::Parser;
use std::net;

// Based on: https://doc.rust-lang.org/std/net/struct.UdpSocket.html
// See also: https://docs.rs/socket2/latest/socket2/struct.Socket.html for lower-level socket API

const MAX_LINE: usize = 1024;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen for connections on
    #[arg(short, long, default_value_t = 12000)]
    port: u16,
}

fn main() {
    // Process commandline arguments
    let args = Args::parse();

    let port = args.port;

    // Create socket and bind to receive messages on specified port
    //let socket = match net::UdpSocket::bind((net::Ipv4Addr::UNSPECIFIED, port)) {
    let socket = match net::UdpSocket::bind((net::Ipv6Addr::UNSPECIFIED, port)) {
        Ok(socket) => socket,
        Err(error) => {
            eprintln!("Error: could not bind to specified port '{}': {}", port, error);
            std::process::exit(1);
        }
    };
    println!("Listening for data on port {}", port);

    // Init buffer to receive into
    let mut buf: [u8; MAX_LINE] = [0; MAX_LINE];

    loop {
        // Receive data from client
        let (bytes, from_addr) = socket.recv_from(&mut buf).unwrap();

        // Print out received message
        println!(
            "Received {bytes:?} bytes from {from_addr:?} {}",
            std::str::from_utf8(&buf).unwrap()
        );

        // Convert received message to upper case (just to show we can process or modify however we
        // want
        for i in 0..bytes {
            if buf[i].is_ascii() {
                buf[i].make_ascii_uppercase();
            }
        }

        // Send modified message to client
        let bytes = socket.send_to(&buf[0..bytes], from_addr).unwrap();
        println!(
            "Sent {bytes:?} bytes to client: {}",
            std::str::from_utf8(&buf).unwrap()
        );
    }
}
