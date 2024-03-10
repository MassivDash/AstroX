use crate::cli::config::create_dotenv::create_dotenv_frontend;
use crate::cli::config::get_config::{get_config, Config, ASTROX_TOML};
use crate::cli::config::toml::read_toml;
use crate::cli::pre_run::npm::checks::NPM;
use crate::cli::utils::terminal::step;
use std::process::Command;

pub fn build_production(config: Config) {
    // Bundle the frontend and wait for the process to finish
    // if the astro build is set to true
    // start the build process

    if config.prod_astro_build {
        // take production build url from config
        let prod_build_url = config.public_keys.public_api_url;

        create_dotenv_frontend(&prod_build_url, "./src/frontend/.env");

        step("Building the frontend package");

        let bundle = Command::new(NPM)
            .arg("run")
            .arg("build")
            .current_dir("./src/frontend")
            .spawn()
            .expect("Failed to bundle the frontend")
            .wait()
            .expect("Failed to bundle the frontend");

        match bundle.success() {
            true => step("Frontend bundled successfully"),
            false => panic!("Failed to bundle the frontend"),
        }
    }

    // Start the backend production server

    step("Building cargo backend production server");

    let cargo_server = Command::new("cargo")
        .current_dir("./src/backend")
        .arg("build")
        .arg("--release")
        .spawn()
        .expect("Failed to start backend production server")
        .wait()
        .expect("Failed to start backend production server");

    match cargo_server.success() {
        true => step("Backend built successfully"),
        false => panic!("Failed to build the backend"),
    }
}

pub fn execute_build() {
    let config = read_toml(&ASTROX_TOML.to_string());
    match config {
        Ok(mut config) => {
            config.env = "prod".to_string();
            build_production(config)
        }
        Err(_) => {
            let mut config = get_config(&vec![]);
            config.env = "prod".to_string();
            build_production(config);
        }
    }
}
