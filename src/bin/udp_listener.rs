use std::{net::UdpSocket, str::from_utf8};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;
    loop {
        // Receives a single datagram message on the socket.
        // If `buf` is too small to hold the msg, it'll be cut off.
        let mut buf = [0; 32];
        let (_, src) = socket.recv_from(&mut buf)?;
        println!("Recv From: {} {}", src, from_utf8(&buf).unwrap());
    }
}
