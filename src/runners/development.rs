use ctrlc::set_handler;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::process::Command;

use super::system_checks::{is_cargo_watch_installed, is_node_installed, NPM};
use super::terminal::{do_splash, step};

pub fn start_development(host: &str, port: &str) {
    // Set the ctrl-c handler to exit the program and clean up orphaned processes
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Check if the user has cargo watch installed, panic and inform the user what to do

    let is_cargo_watch_installed = is_cargo_watch_installed();

    match is_cargo_watch_installed {
                true => step("cargo-watch is installed, you are ready for 🚀"),
                false => panic!("cargo-watch is not installed, please install it by running 'cargo install cargo-watch'"),
            }

    // Check if the user has node installed, panic and inform the user what to do

    let is_node_installed = is_node_installed();

    match is_node_installed {
        true => step("node is installed"),
        false => {
            panic!("node is not installed, please install it by running 'brew install node'")
        }
    }

    // Print the splash screen
    do_splash();

    // Start the backend development server

    step("Starting cargo backend development server");

    let mut cargo_watch = Command::new("cargo")
        .current_dir("./src/backend")
        .arg("watch")
        .arg("-w")
        .arg("./src")
        .arg("-x")
        .arg(format!("run -- --host={} --port={}", host, port))
        .spawn()
        .expect("Failed to start backend development server");

    // Start the frontend development server

    step("Starting astro frontend development server");

    let mut node_watch = Command::new(NPM)
        .arg("run")
        .arg("start")
        .current_dir("./src/frontend")
        .spawn()
        .expect("Failed to start frontend development server");

    // Clean up section for orphaned processes, otherwise cargo watch and node watch will continue to run blocking the ports
    while running.load(Ordering::SeqCst) {}
    step("Cleaning up orphaned processes");

    cargo_watch
        .kill()
        .expect("Failed to kill cargo-watch process");
    node_watch
        .kill()
        .expect("Failed to kill node-watch process");

    step("Exiting");

    std::process::exit(0);
}
