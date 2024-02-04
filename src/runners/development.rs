use ctrlc::set_handler;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::process::Command;
use std::io::Read;

use super::system_checks::{is_cargo_watch_installed, is_node_installed, NPM};
use super::terminal::{do_splash, step};

pub fn start_development(host: &str, port: &str, astro_port: &str) {
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
        .arg(format!("-- --port {}", astro_port))
        .stdout(std::process::Stdio::piped())
        .current_dir("./src/frontend")
        .spawn()
        .expect("Failed to start frontend development server");

    // Watch the std output of astro bundle if std will have "ready" then open the browser to the development server

    let mut buffer = [0; 1024];
    let mut stdout = node_watch.stdout.take().unwrap();
    loop {
        let n = stdout.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        let s = String::from_utf8_lossy(&buffer[..n]);
        print!("{}", s);

        if s.contains("ready") {
            step("Astro is ready, opening the browser");
            Command::new("open")
                .arg(format!("http://localhost:{}", astro_port))
                .spawn()
                .expect("Failed to open the browser");
            break;
        }
    }


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
