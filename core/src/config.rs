use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Result;

use crate::dir_utils::expand_tilde;

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub id: u8,
    pub hostname: String,
    pub port: u32,
}

pub struct ClusterConfig {}

impl ClusterConfig {
    pub fn get_nodepool_data(path: &PathBuf) -> Vec<Node> {
        let configpath = expand_tilde(path).unwrap();
        let mut config = configpath.clone();
        config.push("cluster.config.json");

        let data = fs::read_to_string(config).unwrap();
        let node_pools: Vec<Node> = serde_json::from_str(&data).unwrap();
        node_pools
    }
}
