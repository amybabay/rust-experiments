use clap::Parser;
use std::io::{stdout, Write};
use std::net::{ToSocketAddrs, IpAddr, UdpSocket, Ipv4Addr, Ipv6Addr};

// Based on: https://doc.rust-lang.org/std/net/struct.UdpSocket.html
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

    let server_str = format!("{}:{}", host, port);
    let mut socket_addrs = match server_str.to_socket_addrs() {
        Ok(socket_addrs) => socket_addrs,
        Err(error) => {
            eprintln!("Error: invalid server address '{}': {}", host, error);
            std::process::exit(1);
        }
    };
    let server_addr = socket_addrs.next().unwrap();

    // Choose the appropriate socket type based on the IP version of the destination address and
    // create UDP socket. Binding to unspecified address and port 0, since we don't care what
    // source port is assigned. The OS will assign a random source port
    let socket = match server_addr.ip() {
         IpAddr::V4(_) => UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).unwrap(),
         IpAddr::V6(_) => UdpSocket::bind((Ipv6Addr::UNSPECIFIED, 0)).unwrap(), 
    };

    // Read from keyboard
    print!("Enter your message: ");
    stdout().flush().unwrap(); // force print to screen
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    // Send to server
    let bytes = socket.send_to(input.as_bytes(), format!("{}:{}", host, port)).unwrap();
    println!("Sent {bytes:?} bytes to server: {}", input);

    // Receive reply from server
    let mut buf: [u8; MAX_LINE] = [0; MAX_LINE];
    let (bytes, from_addr) = socket.recv_from(&mut buf).unwrap();

    // Print out received message
    println!(
        "Received {bytes:?} bytes from {} {}",
        from_addr,
        std::str::from_utf8(&buf).unwrap()
    );
}
