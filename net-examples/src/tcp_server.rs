use clap::Parser;
use std::io::{Read, Write};
use std::net;

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

    // Create TCP socket and bind to give port (TcpListener object wraps the actual socket)
    // see: https://doc.rust-lang.org/std/net/struct.TcpListener.html

    // Using IPv6 here allows IPv4 and IPv6 clients
    let listener = net::TcpListener::bind((net::Ipv6Addr::UNSPECIFIED, port)).unwrap();
    //let listener = net::TcpListener::bind((net::Ipv4Addr::UNSPECIFIED, port)).unwrap();
    println!("Listening for data on port {}", port);

    loop {
        // Accept client connection
        let accept_res = listener.accept();
        let (mut socket, addr) = match accept_res {
            Ok((socket, addr)) => (socket, addr),
            Err(e) => panic!("accept failed: {e:?}"),
        };

        // Receive data from client
        let mut buf: [u8; MAX_LINE] = [0; MAX_LINE];
        let bytes = socket.read(&mut buf).unwrap();

        // Print out received message
        println!(
            "Received {bytes:?} bytes from {addr:?} {}",
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
        let bytes = socket.write(&buf[0..bytes]).unwrap();
        println!(
            "Sent {bytes:?} bytes to client: {}",
            std::str::from_utf8(&buf).unwrap()
        );
    }
}
