use control_plane::Node;
use rand::Rng;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::{fs, io};

use futures::FutureExt;
use std::env;
use std::{collections::HashMap, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let data = fs::read_to_string("node_pool.json").await.unwrap();
    let node_pools: Vec<Node> = serde_json::from_str(&data)?;

    println!("Node_Pool: {:?}", node_pools);

    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7878".to_string());

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
