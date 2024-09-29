// lib.rs
pub mod config;
pub mod processors;

pub use processors::process_yaml;
pub use config::load_config_from_file;
