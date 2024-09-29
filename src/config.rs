use std::fs::File;
use std::io::{self, BufReader, Read};
use yaml_rust2::YamlLoader;
use yaml_rust2::Yaml;

pub fn load_config_from_file(config_path: &str) -> io::Result<Yaml> {
    let file = File::open(config_path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let docs = YamlLoader::load_from_str(&contents)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Unable to parse config file"))?;
    Ok(docs.into_iter().next().unwrap_or(Yaml::Null))
}