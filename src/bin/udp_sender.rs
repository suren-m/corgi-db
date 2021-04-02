use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34253")?;
    let dest = "127.0.0.1:34254";

    let f = File::open("words").unwrap();
    let br = BufReader::new(f);
    // about 100K words
    for (count, line) in br.lines().into_iter().enumerate() {
        if let Ok(word) = line {
            socket.send_to(word.as_bytes(), dest)?;
            println!("sending: {} count - {}", word, count);
        }
    }
    Ok(())
}
