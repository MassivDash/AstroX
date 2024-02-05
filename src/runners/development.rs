use ctrlc::set_handler;
use std::io::Read;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::runners::terminal::success;

use super::system_checks::{run_system_checks, NPM};
use super::terminal::{dev_info, do_splash, step, warning};

pub fn start_development(host: &str, port: &str, astro_port: &str) {
    // Set the ctrl-c handler to exit the program and clean up orphaned processes
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Print the splash screen
    do_splash();

    // Check if the user has cargo watch installed, panic and inform the user what to do

    warning("Checking the prerequisites");

    let prod_astro_build = true;

    run_system_checks(prod_astro_build);

    // Check if the port is available for the backend server

    let port = port.parse::<u16>().unwrap();
    let mut listener = std::net::TcpListener::bind(format!("{}:{}", host, port));

    // Loop until you find the port that is available

    let mut new_port = port;

    while listener.is_err() {
        warning(format!("Port {} is not available", new_port).as_str());
        new_port += 1;
        listener = std::net::TcpListener::bind(format!("{}:{}", host, new_port));
    }

    // kill the listener

    drop(listener);

    // Start the backend development server

    dev_info(&host, &new_port);

    let mut cargo_watch = Command::new("cargo")
        .current_dir("./src/backend")
        .arg("watch")
        .arg("-w")
        .arg("./src")
        .arg("-x")
        .arg(format!("run -- --host={} --port={}", host, new_port))
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
            success("Astro is ready, opening the browser");
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
