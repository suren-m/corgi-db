use std::{
    io::stdin,
    process::{exit, Command, Stdio},
    thread,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    nodecount: u8,

    startingport: u16,
}

fn main() {
    let args = Cli::from_args();
    let serverpath = "./target/debug/corgi-server";

    if args.nodecount <= 0 {
        exit(-1);
    }

    // let mut nodes: HashMap<String, Child> = HashMap::new();

    println!("spawning servers");

    let mut current_port = args.startingport;
    for id in 1..=args.nodecount {
        thread::spawn(move || {
            let mut child = Command::new(serverpath)
                .args(&[current_port.to_string(), id.to_string()])
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to execute child");

            let name = format!("Node-{}", current_port);
            // nodes.insert(name.clone(), child);
            println!("waiting on {}", name);
            child.wait().expect("failed to wait on child");
        });
        current_port = current_port + 1;
    }

    loop {
        println!("type 'quit' to exit");
        let mut cmd: String = String::new();
        stdin().read_line(&mut cmd).unwrap();

        // if cmd == "quit" {
        //     for (k, v) in &mut nodes {
        //         println!("killing {}", k);
        //         v.kill().expect("command wasn't running");
        //     }
        //     println!("..done...");
        // }
    }
}
