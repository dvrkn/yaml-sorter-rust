// config.rs
use std::fs::File;
use std::io::{BufReader, Read};
use yaml_rust2::YamlLoader;

pub struct Config {
    pub sort_key: String,
    pub pre_order: Vec<String>,
}

fn load_config_from_file(config_path: &str) -> yaml_rust2::Yaml {
    let file = File::open(config_path).expect("Unable to open config file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Unable to read config file");
    let docs = YamlLoader::load_from_str(&contents).expect("Unable to parse config file");
    docs[0].clone()
}

pub fn set_config(sort_key: String, pre_order: Vec<String>) -> Config {
    Config {
        sort_key,
        pre_order,
    }
}

pub fn init_config(config_path: &str) -> Config {
    let config_yaml = load_config_from_file(config_path);
    set_config(
        config_yaml["sortKey"]
            .as_str()
            .unwrap_or("")
            .to_string(),
        config_yaml["preOrder"]
            .as_vec()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|x| x.as_str().map(|s| s.to_string()))
            .collect(),
    )
}