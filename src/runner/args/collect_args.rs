use std::env;

/// Get the additional arguments from "cargo run"
/// List of arguments
/// Bind actix server to a host, used for development and production
/// --host=127.0.0.1
/// Bind actix server to a port, used for development and production
/// --port=8080
/// Set the environment
/// --env=prod / dev
/// Set the astro development port, in production actix server will serve the frontend build Files
/// --astro-port=4321
/// Switch on / off the production build of the frontend during the production server start
/// --prod-astro-build=true / false
/// Switch on the interactive cli
/// --cli
/// Show the help message
/// --help

pub struct Args {
    pub host: String,
    pub port: String,
    pub env: String,
    pub astro_port: String,
    pub prod_astro_build: bool,
    pub cli: bool,
    pub help: bool,
}

fn split_and_collect(arg: &str) -> &str {
    let split: Vec<&str> = arg.split('=').collect();
    if split.len() == 2 {
        return split[1];
    }
    ""
}

fn parse_to_bool(arg: &str) -> bool {
    match arg {
        "true" => true,
        "false" => false,
        _ => false,
    }
}

pub fn collect_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let mut host = "127.0.0.1";
    let mut port = "8080";
    let mut env = "dev";
    let mut astro_port = "4321";
    let mut prod_astro_build: bool = true;
    let mut cli = false;
    let mut help = false;

    for arg in &args {
        if arg.starts_with("--env=") {
            env = split_and_collect(arg);
        }
        if arg.starts_with("--host=") {
            host = split_and_collect(arg);
        }
        if arg.starts_with("--port=") {
            port = split_and_collect(arg);
        }

        if arg.starts_with("--astro-port=") {
            astro_port = split_and_collect(arg);
        }

        if arg.starts_with("--prod-astro-build=") {
            prod_astro_build = parse_to_bool(arg);
        }

        if arg.starts_with("--cli") {
            cli = true;
        }

        if arg.starts_with("--help") {
            help = true;
        }
    }

    Args {
        host: host.to_string(),
        port: port.to_string(),
        env: env.to_string(),
        astro_port: astro_port.to_string(),
        prod_astro_build,
        cli,
        help,
    }
}
