use std::net::TcpListener;
use std::net::TcpStream;
use std::{
    env,
    io::{BufRead, BufReader, Write},
};
use std::{
    fs::File,
    net::{Ipv4Addr, SocketAddrV4},
};

use std::{io::prelude::*, str::from_utf8};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let port_num: u16 = args[1].to_owned().parse().unwrap();
    let node_id: u16 = args[2].to_owned().parse().unwrap();
    println!("port is {}", port_num);
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, port_num);
    let listener = TcpListener::bind(socket).unwrap();
    let port = listener.local_addr().unwrap();
    println!("Listening on {}, access this port to end the program", port);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, node_id, port_num);
    }
}

fn handle_connection(mut stream: TcpStream, node_id: u16, port_num: u16) {
    let mut incoming_buf = [0; 512];
    stream.read(&mut incoming_buf).unwrap();
    println!("Human says: {}", from_utf8(&incoming_buf).unwrap());

    let f = File::open("words").unwrap();
    let br = BufReader::new(f);
    // about 100K words
    dbg!(
        "node {} on port_num {} sending data to {}",
        node_id,
        port_num,
        stream.peer_addr().unwrap()
    );
    for (_count, line) in br.lines().into_iter().enumerate() {
        if let Ok(word) = line {
            stream.write(format!("{} \n", word).as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
