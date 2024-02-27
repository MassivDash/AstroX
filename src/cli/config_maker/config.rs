use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub env: String,
    pub astro_port: Option<u16>,
    pub prod_astro_build: bool,
}
