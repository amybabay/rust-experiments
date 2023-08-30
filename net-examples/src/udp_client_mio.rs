use clap::Parser;
use std::io::{stdout, Write};
use std::net::{ToSocketAddrs, IpAddr, Ipv4Addr, Ipv6Addr};
use mio::net::{UdpSocket};
use mio::unix::{SourceFd};
use std::io;

const MAX_LINE: usize = 1024;
const STDIN_FD: mio::Token = mio::Token(0);
const UDP_SOCKET: mio::Token = mio::Token(1);

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
    let mut socket = match server_addr.ip() {
         IpAddr::V4(_) => UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0).into()).unwrap(),
         IpAddr::V6(_) => UdpSocket::bind((Ipv6Addr::UNSPECIFIED, 0).into()).unwrap(), 
    };

    // Mio setup
    let mut poll = mio::Poll::new().unwrap();
    let mut events = mio::Events::with_capacity(2);

    // Register socket for communicating with server with mio
    poll.registry().register(&mut socket, UDP_SOCKET, mio::Interest::READABLE).unwrap();

    // Register stdin with mio
    let stdin_raw = 0;
    let mut stdin_fd = SourceFd(&stdin_raw);
    poll.registry().register(&mut stdin_fd, STDIN_FD, mio::Interest::READABLE).unwrap();

    // Print prompt to screen
    print!("Enter your message: ");
    stdout().flush().unwrap(); // force print to screen

    // Setup buffer for receiving server responses
    let mut buf: [u8; MAX_LINE] = [0; MAX_LINE];

    loop {
        // Poll to see if any events are ready
        if let Err(err) = poll.poll(&mut events, None) {
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            std::process::exit(1);
        }

        for event in events.iter() {
            match event.token() {
                // Ready to read from keyboard
                STDIN_FD => {
                    // Read from keyboard
                    let mut input = String::new();
                    let stdin = std::io::stdin();
                    stdin.read_line(&mut input).unwrap();

                    // Send to server
                    let bytes = socket.send_to(input.as_bytes(), server_addr).unwrap();
                    println!("Sent {bytes:?} bytes to server: {}", input);
                }

                // Ready to read from server
                UDP_SOCKET => loop {
                    // Try to receive message from server
                    match socket.recv_from(&mut buf) {
                        Ok((bytes, from_addr)) => {
                            // Print out received message
                            println!(
                                "Received {bytes:?} bytes from {} {}",
                                from_addr,
                                std::str::from_utf8(&buf[0..bytes]).unwrap()
                            );
                        }
                        Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                            // done receiving from server for now; no messages waiting
                            break;
                        }
                        Err(err) => {
                            // something went wrong! just exit
                            eprintln!("Error: recv_from failed with error: {}", err);
                            std::process::exit(1);
                        }
                    }
                }

                // Should never get something that doesn't match one of our registered tokens
                _ => unreachable!(),
            }
        }
    }
}
