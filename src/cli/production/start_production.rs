use crate::cli::pre_run::npm::NPM;
use crate::cli::utils::terminal::step;
use std::process::Command;

/// Start the production server
/// The production server will start the actix backend server
/// The production server will also bundle the frontend

pub fn start_production(host: String, port: String, prod_astro_build: bool) {
    // Bundle the frontend and wait for the process to finish
    // if the astro build is set to true
    // start the build process

    if prod_astro_build {
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

    Command::new("cargo")
        .current_dir("./src/backend")
        .arg("run")
        .arg("--release")
        .arg("--")
        .arg(format!("--host={}", host))
        .arg(format!("--port={}", port))
        .spawn()
        .expect("Failed to start backend production server");
}
