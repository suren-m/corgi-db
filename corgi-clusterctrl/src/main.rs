use rand::prelude::*;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    process::{exit, Command, Stdio},
    str::from_utf8,
    thread,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    nodecount: u8,

    startingport: u16,

    #[structopt(default_value = "./target/debug/corgi-server")]
    serverpath: String,
}

struct Node {
    id: u8,
    port: u16,
}

impl Node {
    fn new(id: u8, port: u16) -> Self {
        Node { id, port }
    }
}

fn main() {
    let args = Cli::from_args();

    if args.nodecount <= 0 {
        exit(1);
    }

    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 7878);
    let listener = TcpListener::bind(socket).unwrap();

    let mut nodepool: Vec<Node> = spawn_nodes(args);
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        if let Some(dest_node) = nodepool.pop() {
            let dest_addr = format!("localhost:{}", dest_node.port);
            handle_connection(stream, dest_addr);
            nodepool.push(dest_node);
        } else {
            stream
                .write(format!("No nodes available. try again later.").as_bytes())
                .unwrap();
            stream.flush().unwrap();
        }
    }

    loop {
        println!("type 'quit' to exit");
        let mut cmd: String = String::new();
        stdin().read_line(&mut cmd).unwrap();
    }
}

fn spawn_nodes(args: Cli) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    println!("spawning nodes");

    let mut current_port = args.startingport;
    for id in 1..=args.nodecount {
        let serverpath = args.serverpath.to_owned();
        thread::spawn(move || {
            let mut child = Command::new(serverpath)
                .args(&[current_port.to_string(), id.to_string()])
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute child");

            let name = format!("Node-{}", current_port);
            println!("waiting on {}", name);
            child.wait().expect("failed to wait on child");
        });

        nodes.push(Node::new(id, current_port));
        current_port = current_port + 1;
    }
    nodes
}

fn handle_connection(mut stream: TcpStream, dest: String) {
    let mut incoming_buf = [0; 512];
    stream.read(&mut incoming_buf).unwrap();
    println!("Human says: {}", from_utf8(&incoming_buf).unwrap());

    println!("connecting to {}", dest);
    match TcpStream::connect(dest.clone()) {
        Ok(mut down_stream) => {
            let msg = b"Who's my good dog??";
            down_stream.write(msg).unwrap();

            let br = BufReader::new(down_stream);
            dbg!("response from downstream:{}", dest);
            for (_, line) in br.lines().into_iter().enumerate() {
                if let Ok(word) = line {
                    stream.write(format!("{} \n", word).as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }
        }
        Err(_) => println!("can't connect"),
    }
}
