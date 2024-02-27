use crate::cli::{
    config::toml::create_toml_file,
    pre_run::{git_hooks::copy_git_hooks, system_checks::run_system_checks},
    utils::terminal::{help, step},
};

use super::cmd_list::{check_for_cli_cmds, CliCmds};

pub fn execute_cmd(args: &Vec<String>) {
    let cmd = check_for_cli_cmds(args);
    if cmd != CliCmds::Run {
        match cmd {
            CliCmds::Help => {
                help();
                std::process::exit(0);
            }
            CliCmds::SyncGitHooks => {
                // Copy the git hooks to the .git/hooks folder
                // Enjoy pre-commit, pre-push and commit-msg hooks that will help you to maintain the code quality
                step("Syncing the git hooks");
                copy_git_hooks();
                std::process::exit(0);
            }
            CliCmds::CreateToml => {
                create_toml_file("Astrox.toml".to_string())
                    .expect("Failed to create Astrox.toml file");
                std::process::exit(0);
            }
            CliCmds::Interactive => print!("Interactive mode is not yet implemented"),
            CliCmds::SystemCheck => {
                run_system_checks(&"dev".to_string(), false);
                std::process::exit(0);
            }

            CliCmds::Run => {}
        }
    }
}
