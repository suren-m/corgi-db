use rand::Rng;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::{collections::HashMap, error::Error};
use std::{env, net::SocketAddrV4};

use corgi_clusterctrl::Node;
use std::process::{Command, Stdio};
use std::thread;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = "2")]
    nodecount: u8,

    #[structopt(default_value = "localhost")]
    hostname: String,

    #[structopt(default_value = "5510")]
    startingport: u16,

    #[structopt(default_value = "./target/debug/corgi-server")]
    serverpath: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7878".to_string());

    println!("Listening on: {}", listen_addr);

    let node_pools = spawn_nodes(args);
    let mut sticky_sessions: HashMap<String, String> = HashMap::new();
    let mut rng = rand::thread_rng();

    let listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, caller)) = listener.accept().await {
        let mut backend_addr = String::new();
        let caller_addr = caller.to_string();
        println!("Request from {}", caller_addr);

        if let Some(proxy) = sticky_sessions.get(&caller_addr) {
            backend_addr = proxy.to_owned();
            println!("sticky session to {}", backend_addr);
        } else {
            let rand_num = rng.gen_range(0..node_pools.len());
            let rand_node = &node_pools[rand_num];
            backend_addr = format!("{}:{}", rand_node.hostname, rand_node.port);
            println!("Transfer to {}", backend_addr);
        }
        sticky_sessions.insert(caller_addr, backend_addr.to_owned());
        let transfer = transfer(inbound, backend_addr).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
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

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
