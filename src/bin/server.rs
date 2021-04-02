use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::{thread, time};

use std::{io::prelude::*, str::from_utf8};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut incoming_buf = [0; 512];
    stream.read(&mut incoming_buf).unwrap();
    println!("Human says: {}", from_utf8(&incoming_buf).unwrap());

    let f = File::open("words").unwrap();
    let br = BufReader::new(f);
    // about 100K words
    for (count, line) in br.lines().into_iter().enumerate() {
        if let Ok(word) = line {
            stream.write(format!("{} \n", word).as_bytes()).unwrap();
            stream.flush().unwrap();
            dbg!(count);
        }
    }
}
