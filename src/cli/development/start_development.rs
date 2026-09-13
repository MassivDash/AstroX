use crate::cli::config::create_dotenv::create_dotenv_frontend;
use crate::cli::config::get_config::Config;
use crate::cli::pre_run::npm::checks::NPM;
use crate::cli::utils::logs::{
    handle_actix_line, handle_astro_line, stream_lines, wait_until_ready, RealBrowserOpener,
};
use crate::cli::utils::ports::bind_available_port;
use crate::cli::utils::server_args::BackendArgs;
use crate::cli::utils::services::terminate_services;
use crate::cli::utils::terminal::step;
use ctrlc::set_handler;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

/// Decide whether the monitor loop should stop.
/// It stops on ctrl-c, once every child process has exited, or once every log
/// reading thread has finished because its stream was closed.
pub fn should_stop_monitoring(
    running: bool,
    all_children_exited: bool,
    all_threads_finished: bool,
) -> bool {
    !running || all_children_exited || all_threads_finished
}

/// Spawn `cargo watch` on the backend so it restarts on every source change.
fn spawn_backend_watch(config: &Config, port: u16) -> Child {
    let watch_command = BackendArgs {
        host: &config.host,
        port,
        env: None,
        cors_url: None,
        cookie_domain: config.cookie_domain.as_deref(),
    }
    .to_watch_command();

    let mut cargo_cmd = Command::new("cargo");
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cargo_cmd.process_group(0);
    }

    cargo_cmd
        .current_dir("./src/backend")
        .arg("watch")
        .arg("-w")
        .arg("./src")
        .arg("-x")
        .arg(watch_command)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start backend development server")
}

/// Spawn the astro development server on `astro_port`.
fn spawn_frontend(astro_port: u16) -> Child {
    let mut node_cmd = Command::new(NPM);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        node_cmd.process_group(0);
    }

    node_cmd
        .arg("run")
        .arg("start")
        .arg("--")
        .arg("--port")
        .arg(astro_port.to_string())
        .stdout(std::process::Stdio::piped())
        .current_dir("./src/frontend")
        .spawn()
        .expect("Failed to start frontend development server")
}

/// Start the development server
/// The development server will start the actix backend server and the astro frontend server
/// The development server will also check if the port is available for the backend server, and loop until it finds the available port
/// The development server will also clean up the orphaned processes, otherwise cargo watch and node watch will continue to run, blocking the ports.
pub fn start_development(config: Config) {
    // Set the ctrl-c handler to exit the program and clean up orphaned processes
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Check if the ports are available, the listeners are held until every port
    // has been picked so that two services can never land on the same port
    let (port, rust_port_listener) =
        bind_available_port(&config.host, config.port.unwrap_or(8080), "Port");
    let (astro_port, astro_port_listener) =
        bind_available_port(&config.host, config.astro_port.unwrap_or(5431), "Port");

    // kill the listeners
    drop(rust_port_listener);
    drop(astro_port_listener);

    // Create the host env for astro to call the actix backend server
    create_dotenv_frontend(
        &format!("http://{}:{}/api", config.host, port),
        "./src/frontend/.env",
    );

    // Start the backend development server
    step("Start the actix backend development server");
    let mut cargo_watch = spawn_backend_watch(&config, port);

    // Wait for the backend development server to start and set up continuous reading
    let stdout_rust = cargo_watch.stdout.take().unwrap();
    let rust_ready = Arc::new(AtomicBool::new(false));
    let rust_ready_clone = rust_ready.clone();
    let host_clone = config.host.clone();

    // Spawn thread to read Rust backend logs continuously
    let rust_handle = thread::spawn(move || {
        stream_lines(stdout_rust, |line| {
            handle_actix_line(line, &host_clone, port, &rust_ready_clone)
        });
    });

    // Wait for Rust backend to be ready before starting frontend
    wait_until_ready(&rust_ready);

    // Start the frontend development server
    step("Starting astro frontend development server");
    let mut node_watch = spawn_frontend(astro_port);

    // Watch the std output of astro bundle if std will have "ready" then open the browser to the development server
    let stdout_node = node_watch.stdout.take().unwrap();
    let astro_ready = Arc::new(AtomicBool::new(false));
    let astro_ready_clone = astro_ready.clone();

    // Spawn thread to read Astro frontend logs continuously
    let astro_handle = thread::spawn(move || {
        stream_lines(stdout_node, |line| {
            handle_astro_line(line, astro_port, &astro_ready_clone, &RealBrowserOpener)
        });
    });

    // Main loop: keep the process alive and monitor services
    loop {
        sleep(Duration::from_millis(100));

        let all_children_exited = cargo_watch.try_wait().unwrap_or(None).is_some()
            && node_watch.try_wait().unwrap_or(None).is_some();

        let all_threads_finished = rust_handle.is_finished() && astro_handle.is_finished();

        if should_stop_monitoring(
            running.load(Ordering::SeqCst),
            all_children_exited,
            all_threads_finished,
        ) {
            break;
        }
    }

    step("Cleaning up orphaned processes");

    terminate_services(
        &mut [&mut cargo_watch, &mut node_watch],
        "target/debug/backend",
    );

    step("Exiting");

    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_stop_monitoring_on_ctrl_c() {
        assert!(should_stop_monitoring(false, false, false));
    }

    #[test]
    fn test_should_stop_monitoring_when_all_children_exited() {
        assert!(should_stop_monitoring(true, true, false));
    }

    #[test]
    fn test_should_stop_monitoring_when_all_threads_finished() {
        assert!(should_stop_monitoring(true, false, true));
    }

    #[test]
    fn test_keep_monitoring_while_the_services_are_alive() {
        assert!(!should_stop_monitoring(true, false, false));
    }
}
