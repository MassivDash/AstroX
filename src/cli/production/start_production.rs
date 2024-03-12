use crate::cli::config::create_dotenv::create_dotenv_frontend;
use crate::cli::config::get_config::{get_config, Config, ASTROX_TOML};
use crate::cli::config::toml::read_toml;
use crate::cli::pre_run::npm::checks::NPM;
use crate::cli::utils::terminal::step;
use ctrlc::set_handler;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

/// Start the production server
/// The production server will start the actix backend server
/// The production server will also bundle the frontend

pub fn start_production(config: Config) {
    // Bundle the frontend and wait for the process to finish
    // if the astro build is set to true
    // start the build process

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    if config.prod_astro_build {
        // take production build url from config
        let prod_build_url = config.public_keys.public_api_url;

        create_dotenv_frontend(&prod_build_url, "./src/frontend/.env");

        step("Bundling the frontend");

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

    step("Starting cargo backend production server");

    let mut cargo_server = Command::new("cargo")
        .current_dir("./src/backend")
        .arg("run")
        .arg("--release")
        .arg("--")
        .arg(format!("--host={}", config.host))
        .arg(format!("--port={}", config.port.unwrap_or(8080)))
        .arg(format!("--env={}", config.env))
        .spawn()
        .expect("Failed to start backend production server");

    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(100));
    }
    step("Cleaning up orphaned processes");

    cargo_server
        .kill()
        .expect("Failed to kill cargo-watch process");

    step("Exiting");

    std::process::exit(0);
}

pub fn execute_serve() {
    let config = read_toml(&ASTROX_TOML.to_string());
    match config {
        Ok(mut config) => {
            config.env = "prod".to_string();
            start_production(config);
        }
        Err(_) => {
            let mut config = get_config(&vec![]);
            config.env = "prod".to_string();
            start_production(config);
        }
    }
}
