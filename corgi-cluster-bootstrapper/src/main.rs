use std::{
    io::{self, Read},
    process::{Command, Stdio},
    thread,
};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use corgi_cluster_bootstrapper::Node;
use serde_json::json;
use std::fs::OpenOptions;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "2")]
    nodecount: u8,

    #[structopt(default_value = "localhost")]
    hostname: String,

    #[structopt(default_value = "5515")]
    startingport: u16,

    #[structopt(default_value = "./target/debug/corgi-server")]
    serverpath: String,
}

fn main() {
    let args = Cli::from_args();
    let node_pool = json!(spawn_nodes(args));
    println!("{}", node_pool.to_string());

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("node_pool.json")
        .unwrap();

    file.write_all(node_pool.to_string().as_bytes()).unwrap();

    loop {
        println!("..waiting...");
        let mut buffer = String::new();
        let mut stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_to_string(&mut buffer).unwrap();
    }
}

fn spawn_nodes(args: Cli) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    println!("spawning nodes");

    let dest_hostname = args.hostname;
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

        nodes.push(Node::new(id, dest_hostname.clone(), current_port));
        current_port = current_port + 1;
    }
    nodes
}
