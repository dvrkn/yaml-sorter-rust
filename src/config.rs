use std::fs::File;
use std::io::Read;
use yaml_rust2::{YamlLoader, Yaml};

pub struct Config {
    pub sort_key: String,
    pub pre_order: Vec<String>
}

fn set_config(sort_key: String, pre_order: Vec<String>) -> Config {
    Config {
        sort_key,
        pre_order
    }
}

pub fn init_config() -> Config {
    let config_yaml = load_config_from_file();
    set_config(
        config_yaml["sortKey"].as_str().unwrap().to_string(),
        config_yaml["preOrder"].as_vec().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect()
    )
}

pub fn init_test_config(mock_config: &str) -> Config {
    let config_yaml = YamlLoader::load_from_str(mock_config).unwrap();
    let config_doc = &config_yaml[0];
    set_config(
        config_doc["sortKey"].as_str().unwrap().to_string(),
        config_doc["preOrder"].as_vec().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect()
    )
}

fn load_config_from_file() -> Yaml {
    let mut file = File::open("config.yaml").expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read config file");
    let docs = YamlLoader::load_from_str(&contents).expect("Unable to parse config file");
    docs[0].clone()
}
