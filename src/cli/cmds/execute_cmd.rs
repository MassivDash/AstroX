use crate::cli::{
    config::toml::create_toml_file,
    pre_run::{
        system_checks::run_system_checks,
        utils::git_hooks::{copy_git_hooks, remove_git_hooks},
    },
    utils::terminal::{help, step},
};

use super::{
    cmd_list::{check_for_cli_cmds, CliCmds},
    interactive::start_interactive,
};

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
            CliCmds::RemoveGitHooks => {
                // Remove the git hooks from the .git/hooks folder
                // This will remove the pre-commit, pre-push and commit-msg hooks
                step("Removing the git hooks");
                remove_git_hooks();
                std::process::exit(0);
            }
            CliCmds::CreateToml => {
                create_toml_file("Astrox.toml".to_string())
                    .expect("Failed to create Astrox.toml file");
                std::process::exit(0);
            }
            CliCmds::Interactive => start_interactive(),
            CliCmds::SystemCheck => {
                run_system_checks("dev");
                std::process::exit(0);
            }

            CliCmds::Run => {}
        }
    }
}
