use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "localhost:7878")]
    dest: String,
}
fn main() {
    let args = Cli::from_args();
    println!("connecting to {}", args.dest);
    match TcpStream::connect(args.dest) {
        Ok(mut stream) => {
            let msg = b"Get some data";
            stream.write(msg).unwrap();

            let br = BufReader::new(stream);
            // don't read all at once..
            for (count, line) in br.lines().into_iter().enumerate() {
                if let Ok(word) = line {
                    println!("{} - count= {}", word, count);
                }
            }
        }
        Err(_) => println!("can't connect"),
    }
}
