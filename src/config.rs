extern crate yaml_rust;

use clap::Parser;
use std::fs::File;
use std::{io::Read, path::PathBuf};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Parser)]
struct CLI {
    #[arg(short, long, help = "yaml config file")]
    config: PathBuf,
}

pub fn get_config() -> Vec<Yaml> {
    let cli = CLI::parse();
    let mut config_str = String::new();
    let mut config_file = File::open(cli.config).expect("No config file found at location");
    config_file
        .read_to_string(&mut config_str)
        .expect("Failed to read config file");

    let config =
        YamlLoader::load_from_str(&config_str).expect("Failed to parse yaml from config file");

    config
}
