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

/// List of commands
/// --help
/// --sync-git-hooks
/// --create-toml

#[derive(Debug, PartialEq)]
pub enum CliCmds {
    Help,
    SyncGitHooks,
    CreateToml,
    Interactive,
    SystemCheck,
    Run,
}

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

pub fn check_for_cli_cmds(args: &Vec<String>) -> CliCmds {
    for arg in args {
        match arg.as_str() {
            s if s.starts_with("--help") => return CliCmds::Help,
            s if s.starts_with("--sync-git-hooks") => return CliCmds::SyncGitHooks,
            s if s.starts_with("--create-toml") => return CliCmds::CreateToml,
            s if s.starts_with("--interactive") => return CliCmds::Interactive,
            s if s.starts_with("--system-check") => return CliCmds::SystemCheck,
            _ => continue,
        }
    }
    CliCmds::Run
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

    #[test]
    fn test_collect_config_args() {
        let config = Config {
            env: "".to_string(),
            host: "".to_string(),
            port: None,
            astro_port: None,
            prod_astro_build: false,
        };

        let args = vec![
            "--env=prod".to_string(),
            "--host=127.0.0.1".to_string(),
            "--port=8080".to_string(),
            "--astro-port=4321".to_string(),
            "--prod-astro-build=true".to_string(),
        ];

        let expected_config = Config {
            env: "prod".to_string(),
            host: "127.0.0.1".to_string(),
            port: Some(8080),
            astro_port: Some(4321),
            prod_astro_build: true,
        };

        assert_eq!(collect_config_args(config, &args), expected_config);
    }

    #[test]
    fn test_check_if_help_always_executed() {
        let mut args = vec!["--invalid-arg".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Run);
        args.push("--help".to_string());
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Help);
        args.push("--create-toml".to_string());
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Help);
    }
}
