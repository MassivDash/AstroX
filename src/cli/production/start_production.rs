use crate::cli::config::create_dotenv::create_dotenv_frontend;
use crate::cli::config::get_config::{get_prod_config, Config};
use crate::cli::pre_run::npm::checks::NPM;
use crate::cli::production::build_production::{BuildRunner, RealBuildRunner};
use crate::cli::utils::ports::bind_available_port;
use crate::cli::utils::server_args::BackendArgs;
use crate::cli::utils::services::terminate_services;
use crate::cli::utils::terminal::step;
use ctrlc::set_handler;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

/// What the production monitor loop should do on the current tick.
#[derive(Debug, PartialEq, Eq)]
pub enum MonitorAction {
    /// Nothing happened, keep watching the services.
    KeepRunning,
    /// Stop the server.
    Stop,
    /// The backend died on its own while we are still running, bring it back.
    RestartBackend,
}

/// Decide what to do with the production services on the current tick.
///
/// `backend_exit` is `None` while the backend is still alive, otherwise it
/// carries whether the backend exited successfully.
pub fn next_monitor_action(running: bool, backend_exit: Option<bool>) -> MonitorAction {
    if !running {
        return MonitorAction::Stop;
    }

    match backend_exit {
        // The backend crashed while we are still up, restart it
        Some(false) => MonitorAction::RestartBackend,
        // A clean backend exit means we are done
        Some(true) => MonitorAction::Stop,
        None => MonitorAction::KeepRunning,
    }
}

/// Spawn the release build of the actix backend in its own process group.
fn spawn_backend(config: &Config, port: u16, failure_message: &str) -> Child {
    let args = BackendArgs {
        host: &config.host,
        port,
        env: Some(&config.env),
        cors_url: Some(&config.cors_url),
        cookie_domain: config.cookie_domain.as_deref(),
    }
    .to_args();

    let mut cargo_command = Command::new("cargo");
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cargo_command.process_group(0);
    }

    cargo_command
        .current_dir("./src/backend")
        .arg("run")
        .arg("--release")
        .arg("--")
        .args(args)
        .spawn()
        .expect(failure_message)
}

/// Start the production server
/// The production server will start the actix backend server
/// The production server will also bundle the frontend
pub fn start_production(config: Config) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Check if the port is available
    let (port, rust_port_listener) =
        bind_available_port(&config.host, config.port.unwrap_or(8080), "Port");

    // kill the listener
    drop(rust_port_listener);

    if config.prod_astro_build {
        // take production build url from config
        create_dotenv_frontend(&config.public_keys.public_api_url, "./src/frontend/.env");

        step("Bundling the frontend");

        match RealBuildRunner.run(
            NPM,
            &["run", "build"],
            "./src/frontend",
            "Failed to bundle the frontend",
        ) {
            true => step("Frontend bundled successfully"),
            false => panic!("Failed to bundle the frontend"),
        }
    }

    // Start the backend production server
    step("Starting cargo backend production server");

    let mut cargo_server =
        spawn_backend(&config, port, "Failed to start backend production server");

    // Main loop: keep the process alive and monitor service
    loop {
        sleep(Duration::from_millis(100));

        let backend_exit = match cargo_server.try_wait() {
            Ok(Some(status)) => Some(status.success()),
            _ => None,
        };

        match next_monitor_action(running.load(Ordering::SeqCst), backend_exit) {
            MonitorAction::KeepRunning => continue,
            MonitorAction::Stop => break,
            MonitorAction::RestartBackend => {
                step("Backend production server exited, restarting...");

                cargo_server =
                    spawn_backend(&config, port, "Failed to restart backend production server");
            }
        }
    }

    step("Cleaning up orphaned processes");

    terminate_services(&mut [&mut cargo_server], "target/release/backend");

    step("Exiting");

    std::process::exit(0);
}

pub fn execute_serve() {
    start_production(get_prod_config());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitor_keeps_running_while_everything_is_alive() {
        assert_eq!(next_monitor_action(true, None), MonitorAction::KeepRunning);
    }

    #[test]
    fn test_monitor_stops_on_ctrl_c() {
        assert_eq!(next_monitor_action(false, None), MonitorAction::Stop);
    }

    #[test]
    fn test_monitor_restarts_a_crashed_backend() {
        assert_eq!(
            next_monitor_action(true, Some(false)),
            MonitorAction::RestartBackend
        );
    }

    #[test]
    fn test_monitor_stops_on_a_clean_backend_exit() {
        assert_eq!(next_monitor_action(true, Some(true)), MonitorAction::Stop);
    }

    #[test]
    fn test_a_crashed_backend_does_not_restart_after_ctrl_c() {
        assert_eq!(next_monitor_action(false, Some(false)), MonitorAction::Stop);
    }
}
