use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: u8,
    pub hostname: String,
    pub port: u16,
}

impl Node {
    pub fn new(id: u8, hostname: String, port: u16) -> Self {
        Node { id, hostname, port }
    }
}
