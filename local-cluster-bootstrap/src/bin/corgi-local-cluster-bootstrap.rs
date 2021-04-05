use core::{config::ClusterConfig, dir_utils::expand_tilde};
use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use local_cluster_bootstrap::Node;
use serde_json::json;
use std::fs::OpenOptions;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "2")]
    nodecount: u8,

    #[structopt(short, long, default_value = "localhost")]
    hostname: String,

    #[structopt(short, long, default_value = "5515")]
    portstart: u16,

    #[structopt(short, long, default_value = "~/.corgi", parse(from_os_str))]
    configpath: PathBuf,

    #[structopt(short, long, default_value = "~/.corgi/data", parse(from_os_str))]
    datapath: PathBuf,

    #[structopt(short, long, default_value = "~/.cargo/bin", parse(from_os_str))]
    serverpath: PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let configpath = expand_tilde(&args.configpath).unwrap();
    fs::create_dir_all(&configpath).unwrap();

    let mut config = configpath.clone();
    config.push(ClusterConfig::get_config_filename());

    let mut cluster_config = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config)
        .unwrap();

    let node_pool = json!(spawn_nodes(args));
    println!("{}", node_pool.to_string());

    cluster_config
        .write_all(node_pool.to_string().as_bytes())
        .unwrap();

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
    let mut current_port = args.portstart;
    for id in 1..=args.nodecount {
        let mut serverpath = expand_tilde(&args.serverpath).unwrap();
        serverpath.push("corgi-server");
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
