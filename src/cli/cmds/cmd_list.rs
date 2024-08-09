#[derive(Debug, PartialEq)]
pub enum CliCmds {
    Help,
    SyncGitHooks,
    RemoveGitHooks,
    CreateToml,
    Interactive,
    SystemCheck,
    Run,
    Build,
    Test,
    Serve,
}

pub fn check_for_cli_cmds(args: &Vec<String>) -> CliCmds {
    for arg in args {
        match arg.as_str() {
            s if s.starts_with("--help") => return CliCmds::Help,
            s if s.starts_with("--sync-git-hooks") => return CliCmds::SyncGitHooks,
            s if s.starts_with("--create-toml") => return CliCmds::CreateToml,
            s if s.starts_with("--interactive") => return CliCmds::Interactive,
            s if s.starts_with("--system-check") => return CliCmds::SystemCheck,
            s if s.starts_with("--remove-git-hooks") => return CliCmds::RemoveGitHooks,
            s if s.starts_with("--build") => return CliCmds::Build,
            s if s.starts_with("--test") => return CliCmds::Test,
            s if s.starts_with("--serve") => return CliCmds::Serve,
            _ => continue,
        }
    }
    CliCmds::Run
}
