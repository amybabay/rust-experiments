use clap::Parser;
use mio::net;
use std::time;
use std::io;

// Based on Mio library (provides similar functionality to select/epoll, but in platform
// independent way
// https://docs.rs/mio/0.8.8/mio/
// https://github.com/tokio-rs/mio/blob/master/examples/udp_server.rs

const MAX_LINE: usize = 1024;

// Mio: A token to allow us to identify which event is for the `UdpSocket`.
const UDP_SOCKET: mio::Token = mio::Token(0);

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

    // Create socket and bind to receive messages on specified port (note: uses mio::net to create
    // socket, but std::net to get the unspecified IPv6 addr constant). For mio::net, need to
    // explicitly use into() to convert tuple to SocketAddr
    let mut socket = match net::UdpSocket::bind((std::net::Ipv6Addr::UNSPECIFIED, port).into()) {
        Ok(socket) => socket,
        Err(error) => {
            eprintln!("Error: could not bind to specified port '{}': {}", port, error);
            std::process::exit(1);
        }
    };
    println!("Listening for data on port {}", port);

    // Mio setup - create poll instance
    let mut poll = mio::Poll::new().unwrap();
    // Create storage for events. Since we will only register a single socket, a
    // capacity of 1 will do.
    let mut events = mio::Events::with_capacity(1);

    // Register our socket with token defined above. We want to be notified when there is data to
    // read (i.e. the socket is READABLE)
    poll.registry()
        .register(&mut socket, UDP_SOCKET, mio::Interest::READABLE).unwrap();

    // Init buffer to receive into
    let mut buf: [u8; MAX_LINE] = [0; MAX_LINE];
    let mut last_recv_time: Option<time::Instant> = None;

    loop {
        // Poll to check if we have events waiting for us.
        // if let Err(err) = poll.poll(&mut events, None) { // version with no timeout
        if let Err(err) = poll.poll(&mut events, Some(time::Duration::new(10, 0))) { // 10 second timeout
            if err.kind() == io::ErrorKind::Interrupted {
                continue;
            }
            std::process::exit(1);
        }

        // Process all ready events. Note that spurious wakeups are possible, and that we are
        // required to read until we get a WouldBlock error; otherwise, we are not guaranteed to be
        // notified the next time there is data ready to read.
        for event in events.iter() {
            match event.token() {
                UDP_SOCKET => loop {
                    // Try to receive data from the client
                    match socket.recv_from(&mut buf) {
                        // We received data from client with no error
                        Ok ((bytes, from_addr)) => {
                            // Print out received message
                            println!(
                                "Received {bytes:?} bytes from {from_addr:?} {}",
                                std::str::from_utf8(&buf[0..bytes]).unwrap()
                            );

                            // Convert received message to upper case (just to show we can process
                            // or modify however we want
                            for i in 0..bytes {
                                if buf[i].is_ascii() {
                                    buf[i].make_ascii_uppercase();
                                }
                            }

                            // Send modified message to client
                            let bytes = socket.send_to(&buf[0..bytes], from_addr).unwrap();
                            println!(
                                "Sent {bytes:?} bytes to client: {}",
                                std::str::from_utf8(&buf[0..bytes]).unwrap()
                            );

                            // Record time
                            last_recv_time = Some(time::Instant::now());
                        }
                        Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                            // If we get a `WouldBlock` error we know our socket
                            // has no more packets queued, so we can return to
                            // polling and wait for some more.
                            break;
                        }
                        Err(error) => {
                            // If it was any other kind of error, something went
                            // wrong and we terminate with an error.
                            eprintln!("Error: recv_from failed with error: {}", error);
                            std::process::exit(1);
                        }
                    } // end recv_from match
                } // end UDP recv loop

                // We only registered one token (UDP_SOCKET), so should never trigger on any other
                _ => unreachable!(),
            } // end event match
        } // end event iteration

        // No events are ready, that means this was a timeout
        if events.is_empty() {
            println!("timeout...nothing recceived for 10 seconds.");
            if let Some(last_recv_time) = last_recv_time {
                println!("Last received message was {} seconds ago", last_recv_time.elapsed().as_secs_f64());
            }
        }
    }
}
