// config.rs
use std::fs::File;
use std::io::{BufReader, Read};
use yaml_rust2::YamlLoader;

pub fn load_config_from_file(config_path: &str) -> yaml_rust2::Yaml {
    let file = File::open(config_path).expect("Unable to open config file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Unable to read config file");
    let docs = YamlLoader::load_from_str(&contents).expect("Unable to parse config file");
    docs[0].clone()
}
