#[cfg(test)]
mod tests {
    use crate::cli::cmds::cmd_list::{check_for_cli_cmds, CliCmds};

    #[test]
    fn test_check_for_cli_cmds_help() {
        let args = vec!["--help".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Help);
    }

    #[test]
    fn test_check_for_cli_cmds_build() {
        let args = vec!["--build".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Build);
    }

    #[test]
    fn test_check_for_cli_cmds_serve() {
        let args = vec!["--serve".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Serve);
    }

    #[test]
    fn test_check_for_cli_cmds_test() {
        let args = vec!["--test".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Test);
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
    fn test_check_for_cli_cmds_remove() {
        let args = vec!["--remove-git-hooks".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::RemoveGitHooks);
    }

    #[test]
    fn test_check_for_cli_cmds_coverage() {
        let args = vec!["--coverage".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Coverage);
    }

    #[test]
    fn test_check_for_cli_cmds_run() {
        let args = vec!["--invalid-arg".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Run);
    }

    #[test]
    fn test_check_for_cli_cmds_multiple_args_invalid() {
        let args = vec!["--invalid-arg".to_string(), "--invalid-arg".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Run);
    }

    #[test]
    fn test_check_for_cli_cmds_none() {
        let args = vec![];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Run);
    }

    #[test]
    fn test_check_for_cli_cmds_multiple_args() {
        let args = vec!["--invalid-arg".to_string(), "--help".to_string()];
        assert_eq!(check_for_cli_cmds(&args), CliCmds::Help);
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
