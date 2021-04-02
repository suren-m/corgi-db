use std::{
    collections::HashMap,
    io::stdin,
    process::{exit, Child, Command, Stdio},
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    nodecount: u8,

    startingport: u16,
}

fn main() {
    let args = Cli::from_args();

    if args.nodecount <= 0 {
        exit(-1);
    }

    let serverpath = "./target/debug/corgi-server";

    let mut nodes: HashMap<String, Child> = HashMap::new();

    println!("spawning servers");

    let mut current_port = args.startingport;
    for _ in 1..=args.nodecount {
        let child = Command::new(serverpath)
            .args(&[current_port.to_string()])
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        let name = format!("Node-{}", current_port);
        nodes.insert(name.clone(), child);
        current_port = current_port + 1;
    }

    // for (k, v) in &mut nodes {
    //     println!("waiting on {}", k);
    //     v.wait().expect("failed to wait on child");
    // }

    loop {
        println!("type 'quit' to exit");
        let mut cmd: String = String::new();
        stdin().read_line(&mut cmd).unwrap();

        if cmd == "quit" {
            for (k, v) in &mut nodes {
                println!("killing {}", k);
                v.kill().expect("command wasn't running");
            }
            println!("..done...");
        }
    }
}
