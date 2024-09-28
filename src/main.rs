// main.rs
use clap::Parser;
use processors::process_yaml;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use crate::config::{load_config_from_file};
use yaml_rust2::YamlLoader;

mod config;
mod processors;

#[derive(Parser)]
#[command(name = "ysr", author, version, about = "YAML Sorter")]
struct Cli {
    /// Modify the original input files with diffs
    #[arg(short = 'i', long = "inplace")]
    inplace: bool,

    /// Path to the YAML file
    #[arg(required = true)]
    input: String,

    /// Path to config
    #[arg(long = "config", default_value = "./config.yaml")]
    config: String,
}

fn main() {
    let cli = Cli::parse();
    let config = load_config_from_file(&cli.config);

    let file = File::open(&cli.input).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).expect("Unable to read file");
    let mut docs = YamlLoader::load_from_str(&contents).expect("Unable to parse file");

    for doc in &mut docs {
        process_yaml(doc, &config);
    }

    let mut out_str = String::new();
    {
        let mut emitter = yaml_rust2::YamlEmitter::new(&mut out_str);
        for doc in &docs {
            emitter.dump(doc).unwrap();
        }
    }

    if cli.inplace {
        let file = File::create(&cli.input).expect("Unable to create file");
        let mut writer = BufWriter::new(file);
        let trimmed = out_str.trim_start_matches("---\n");
        writer.write_all(trimmed.as_bytes()).expect("Unable to write to file");
        writer.write_all(b"\n").expect("Unable to write newline");
    } else {
        println!("{}", out_str);
    }
}