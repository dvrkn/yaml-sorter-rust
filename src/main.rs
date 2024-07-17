mod config;
mod processors;

use processors::process_yaml;
use std::fs::File;
use std::io::{Read, Write};
use crate::config::init_config;

struct Cli {
    action: String,
    path: std::path::PathBuf,
}

fn main() {
    let config = init_config();

    let action = std::env::args().nth(1).expect("no action given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        action,
        path: std::path::PathBuf::from(path),
    };

    let mut file = File::open(&args.path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let mut docs = yaml_rust2::YamlLoader::load_from_str(&contents).expect("Unable to parse file");

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

    if args.action == "i" {
        let mut file = File::create(&args.path).expect("Unable to create file");
        file.write_all(out_str.as_bytes()).expect("Unable to write to file");
    } else {
        println!("{}", out_str);
    }
}