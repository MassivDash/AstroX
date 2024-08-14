use super::get_config::Config;

/// List of config arguments
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
/// Set the public api url, this will be copied over to astro frontend and used for grabbing url to set api base
/// During development this value is being copied into the frontend .env file for building the frontend
/// --set-public-api=https://custom.api/api

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

pub fn collect_config_args(config: Config, args: &Vec<String>) -> Config {
    let mut config = config;

    for arg in args {
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
            config.prod_astro_build = parse_to_bool(split_and_collect(arg).as_str());
        }

        if arg.starts_with("--public-api-url=") {
            config.public_keys.public_api_url = split_and_collect(arg);
        }
    }

    config
}

#[cfg(test)]
mod tests {
    use crate::cli::config::get_config::PublicKeys;

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

    #[test]
    fn test_collect_config_args() {
        let config = Config {
            env: "".to_string(),
            host: "".to_string(),
            port: None,
            astro_port: None,
            prod_astro_build: false,
            public_keys: {
                let public_api_url = "http://localhost:8080/api".to_string();
                PublicKeys { public_api_url }
            },
        };

        let args = vec![
            "--env=prod".to_string(),
            "--host=127.0.0.1".to_string(),
            "--port=8080".to_string(),
            "--astro-port=4321".to_string(),
            "--prod-astro-build=true".to_string(),
            "--public-api-url=https://custom.api/api".to_string(),
        ];

        let expected_config = Config {
            env: "prod".to_string(),
            host: "127.0.0.1".to_string(),
            port: Some(8080),
            astro_port: Some(4321),
            prod_astro_build: true,
            public_keys: {
                let public_api_url = "https://custom.api/api".to_string();
                PublicKeys { public_api_url }
            },
        };

        assert_eq!(collect_config_args(config, &args), expected_config);
    }
}
