use std::env;

use super::get_config::Config;

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

fn split_and_collect(arg: &str) -> String {
    let split: Vec<&str> = arg.split('=').collect();
    if split.len() == 2 {
        return split[1].to_string();
    }
    "".to_string()
}

fn parse_to_bool(arg: &str) -> bool {
    match arg {
        "true" => true,
        "false" => false,
        _ => false,
    }
}

pub fn collect_args(config: Config) -> Config {
    let args: Vec<String> = env::args().collect();

    let mut config = config;

    for arg in &args {
        if arg.starts_with("--env=") {
            config.env = split_and_collect(arg);
        }
        if arg.starts_with("--host=") {
            config.host = split_and_collect(arg)
        }
        if arg.starts_with("--port=") {
            config.port = Some(split_and_collect(arg).parse::<u16>().unwrap_or_default());
        }

        if arg.starts_with("--astro-port=") {
            config.astro_port = Some(split_and_collect(arg).parse::<u16>().unwrap_or_default());
        }

        if arg.starts_with("--prod-astro-build=") {
            config.prod_astro_build = parse_to_bool(arg);
        }
    }

    config
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_and_collect() {
        assert_eq!(split_and_collect("--env=prod"), "prod");
        assert_eq!(split_and_collect("--host=127.0.0.1"), "127.0.0.1");
        assert_eq!(split_and_collect("--port=8080"), "8080");
        assert_eq!(split_and_collect("--astro-port=4321"), "4321");
        assert_eq!(split_and_collect("--prod-astro-build=true"), "true");
        assert_eq!(split_and_collect("--invalid-arg"), "");
    }

    #[test]
    fn test_parse_to_bool() {
        assert_eq!(parse_to_bool("true"), true);
        assert_eq!(parse_to_bool("false"), false);
        assert_eq!(parse_to_bool("invalid"), false);
    }
}
