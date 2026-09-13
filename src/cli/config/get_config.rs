use super::collect_args::collect_config_args;
use super::toml::read_toml;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct Config {
    pub host: String,
    pub port: Option<u16>,
    pub env: String,
    pub astro_port: Option<u16>,
    pub cors_url: String,
    pub prod_astro_build: bool,
    pub cookie_domain: Option<String>,
    pub public_keys: PublicKeys,
}

#[derive(Deserialize, Debug, Serialize, PartialEq)]
pub struct PublicKeys {
    pub public_api_url: String,
}

/// Build the configuration out of `toml_path` (when it can be read) and the cli
/// arguments, which always win over the toml file.
pub fn get_config_from(toml_path: &str, args: &Vec<String>) -> Config {
    let astro_port = 5432;
    let cors_url = format!("http://localhost:{}", astro_port);

    let mut config: Config = Config {
        host: "localhost".to_string(),
        port: Some(8080),
        env: "dev".to_string(),
        prod_astro_build: false,
        astro_port: Some(astro_port),
        cors_url,
        cookie_domain: None,
        public_keys: PublicKeys {
            public_api_url: "http://localhost:8080/api".to_string(),
        },
    };

    if let Ok(toml) = read_toml(&toml_path.to_string()) {
        config = toml;
    }

    config = collect_config_args(config, args);
    config
}

pub fn get_config(args: &Vec<String>) -> Config {
    get_config_from(ASTROX_TOML, args)
}

/// Resolve the configuration used by the production commands.
/// Astrox.toml wins, the default config is the fallback, and the environment is
/// always forced to prod.
pub fn get_prod_config() -> Config {
    let mut config = match read_toml(&ASTROX_TOML.to_string()) {
        Ok(config) => config,
        Err(_) => get_config(&vec![]),
    };

    config.env = "prod".to_string();
    config
}

pub const ASTROX_TOML: &str = "Astrox.toml";
