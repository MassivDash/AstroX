use std::env;

/// Get the additional arguments from "cargo run"

/// List of arguments
/// Bind actix server to a host, used for development and production
/// --host=127.0.0.1

/// Bind actix server to a port, used for development and production
/// --port=8080

/// Set the environment
/// --env = prod / dev

pub struct Args {
    pub host: String,
    pub port: String,
    pub env: String,
}

pub fn collect_args() -> Args {
    let args: Vec<String> = env::args().collect();

    let mut env = "dev";
    let mut host = "127.0.0.1";
    let mut port = 8080;

    for arg in &args {
        if arg.starts_with("--env=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                env = split[1];
            }
        }

        if arg.starts_with("--host=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                host = split[1];
            }
        }

        if arg.starts_with("--port=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                port = split[1].parse::<u16>().unwrap();
            }
        }
    }

    Args {
        host: host.to_string(),
        port: port.to_string(),
        env: env.to_string(),
    }
}
