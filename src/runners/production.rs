use std::process::Command;

use super::system_checks::is_node_installed;
use super::terminal::step;

pub fn start_production(host: &str, port: &str, prod_astro_build: bool) {

    // Check if the user has node installed, panic and inform the user what to do

    let is_node_installed = is_node_installed();

    match is_node_installed {
        true => step("node is installed"),
        false => {
            panic!("node is not installed, panicking")
        }
    }

    // Bundle the frontend and wait for the process to finish
    // if the astro build is set to true
    // start the build process

    if prod_astro_build {
        step("Bundling the frontend");

        let bundle = Command::new("npm")
            .arg("run")
            .arg("build")
            .current_dir("./src/frontend")
            .spawn()
            .expect("Failed to bundle the frontend").wait().expect("Failed to bundle the frontend");

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