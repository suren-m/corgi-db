use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: u8,
    pub hostname: String,
    pub port: u32,
}
