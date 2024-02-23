use std::env;

pub struct Args {
    pub host: String,
    pub port: String,
    pub env: String,
    pub astro_port: String,
    pub prod_astro_build: bool,
}

pub fn collect_args() -> Args {
    // Get the additional arguments from "cargo run
    // List of arguments

    // --host=127.0.0.1
    // --port=8080
    // --env=prod / dev
    // --astro-port=4321
    // --prod-astro-build=true / false

    let args: Vec<String> = env::args().collect();
    let mut host = "127.0.0.1";
    let mut port = "8080";
    let mut env = "dev";
    let mut astro_port = "4321";
    let mut prod_astro_build: bool = true;

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
                port = split[1];
            }
        }

        if arg.starts_with("--astro-port=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                astro_port = split[1];
            }
        }

        if arg.starts_with("--prod-astro-build=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                prod_astro_build = split[1].parse::<bool>().unwrap();
            }
        }
    }

    Args {
        host: host.to_string(),
        port: port.to_string(),
        env: env.to_string(),
        astro_port: astro_port.to_string(),
        prod_astro_build,
    }
}
