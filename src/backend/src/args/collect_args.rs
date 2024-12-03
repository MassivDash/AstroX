/// Get the additional arguments from "cargo run"

/// List of arguments
/// Bind actix server to a host, used for development and production
/// --host=127.0.0.1

/// Bind actix server to a port, used for development and production
/// --port=8080

/// Set the environment
/// --env=prod / dev
///

/// Set the cors origin
/// --cors_url=astrox.spaceout.pl

pub struct Args {
    pub host: String,
    pub port: String,
    pub env: String,
    pub cors_url: String,
}

pub fn collect_args(args: Vec<String>) -> Args {
    let mut env = "dev";
    let mut host = "127.0.0.1";
    let mut port = 8080;
    let mut cors_url = "astrox.spaceout.pl";

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

        if arg.starts_with("--cors_url=") {
            let split: Vec<&str> = arg.split('=').collect();
            if split.len() == 2 {
                cors_url = split[1];
            }
        }
    }

    Args {
        host: host.to_string(),
        port: port.to_string(),
        env: env.to_string(),
        cors_url: cors_url.to_string(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_collect_args_default() {
        let args = collect_args(env::args().collect());

        assert_eq!(args.host, "127.0.0.1");
        assert_eq!(args.port, "8080");
        assert_eq!(args.env, "dev");
    }

    #[test]
    fn test_collect_prod_arg() {
        let test_args = vec![
            "--env=prod".to_string(),
            "--port=4000".to_string(),
            "--host=0.0.0.0".to_string(),
            "--cors_url=spaceout.pl".to_string(),
        ];
        let args = collect_args(test_args);

        assert_eq!(args.host, "0.0.0.0");
        assert_eq!(args.port, "4000");
        assert_eq!(args.env, "prod");
        assert_eq!(args.cors_url, "spaceout.pl");
    }
}
