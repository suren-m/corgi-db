use rand::Rng;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::{fs, io};

use core::{
    config::{ClusterConfig, Node},
    dir_utils::expand_tilde,
};
use futures::FutureExt;
use std::{collections::HashMap, error::Error};
use std::{env, path::PathBuf};

use serde_json::json;
use std::fs::OpenOptions;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long, default_value = "127.0.0.1")]
    hostname: String,

    #[structopt(short, long, default_value = "7878")]
    port: String,

    #[structopt(short, long, default_value = "~/.corgi", parse(from_os_str))]
    configpath: PathBuf,

    #[structopt(short, long, default_value = "~/.corgi/data", parse(from_os_str))]
    datapath: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let node_pool: Vec<Node> = ClusterConfig::get_nodepool_data(&args.configpath);

    println!("Node_Pool: {:?}", node_pool);

    let listen_addr = format!("{}:{}", args.hostname, args.port);
    println!("Listening on: {}", listen_addr);

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
            let rand_num = rng.gen_range(0..node_pool.len());
            let rand_node = &node_pool[rand_num];
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
