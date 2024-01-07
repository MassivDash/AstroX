
use std::process::Command;
use super::terminal::{do_splash, step};

use std::env;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

pub fn is_cargo_watch_installed() -> bool {
    let output = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("cargo-watch")
}

pub fn is_node_installed() -> bool {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("v")
}

pub fn start_development() {

    // Print the splash screen

    do_splash();

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
        false => panic!("node is not installed, please install it by running 'brew install node'"),
    }

    // Start the backend development server

    step("Starting cargo backend development server");

    // print current dir 

    let current_dir = env::current_dir().unwrap();
    println!("The current directory is {}", current_dir.display());


    std::process::Command::new("cargo")
            .arg("watch")
            .arg("-x")
            .arg("run")
            .arg("-w")
            .arg("./src")
            .current_dir("./src/backend")
            .spawn()
            .expect("Failed to start backend development server");

    // Start the frontend development server

    step("Starting astro frontend development server");

    std::process::Command::new(NPM)
            .arg("run")
            .arg("start")
            .current_dir("./src/frontend")
            .spawn()
            .expect("Failed to start frontend development server");

}