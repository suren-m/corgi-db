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
    pub fn get_config_filename() -> String {
        String::from("cluster.config.json")
    }
    pub fn get_nodepool_data(path: &PathBuf) -> Vec<Node> {
        let configpath = expand_tilde(path).unwrap();
        let mut config = configpath.clone();
        config.push(ClusterConfig::get_config_filename());

        let data = fs::read_to_string(config).unwrap();
        let node_pools: Vec<Node> = serde_json::from_str(&data).unwrap();
        node_pools
    }

    pub fn get_words_file() -> PathBuf {
        let datapath = expand_tilde("~/.corgi/data").unwrap();
        let mut words_file = datapath.clone();
        words_file.push("words");
        words_file
    }
}
