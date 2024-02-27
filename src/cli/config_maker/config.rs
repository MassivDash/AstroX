use serde::Deserialize;
use serde::Serialize;

use super::collect_args::collect_args;
use super::toml::read_toml;

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: Option<u16>,
    pub env: String,
    pub astro_port: Option<u16>,
    pub prod_astro_build: bool,
}

pub fn get_config() -> Config {
    // create default config

    let mut config: Config = Config {
        host: "localhost".to_string(),
        port: Some(8080),
        env: "dev".to_string(),
        astro_port: Some(5431),
        prod_astro_build: false,
    };

    let file_name = "Astrox.toml".to_string();

    let toml = read_toml(file_name);

    if toml.is_ok() {
        config = toml.unwrap();
    }

    config = collect_args(config);

    config
}
