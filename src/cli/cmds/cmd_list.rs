#[derive(Debug, PartialEq)]
pub enum CliCmds {
    Help,
    SyncGitHooks,
    CreateToml,
    Interactive,
    SystemCheck,
    Run,
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_for_cli_cmds_help() {
        let args = vec!["--help".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Help);
    }

    #[test]
    fn test_check_for_cli_cmds_sync_git_hooks() {
        let args = vec!["--sync-git-hooks".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::SyncGitHooks);
    }

    #[test]
    fn test_check_for_cli_cmds_create_toml() {
        let args = vec!["--create-toml".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::CreateToml);
    }

    #[test]
    fn test_check_for_cli_cmds_interactive() {
        let args = vec!["--interactive".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Interactive);
    }

    #[test]
    fn test_check_for_cli_cmds_system_check() {
        let args = vec!["--system-check".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::SystemCheck);
    }

    #[test]
    fn test_check_for_cli_cmds_run() {
        let args = vec!["--invalid-arg".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Run);
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
