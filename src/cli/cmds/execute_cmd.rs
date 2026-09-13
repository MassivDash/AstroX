use crate::cli::{
    config::{get_config::ASTROX_TOML, toml::create_toml_file},
    pre_run::{
        system_checks::run_system_checks,
        utils::git_hooks::{copy_git_hooks, remove_git_hooks},
    },
    production::{build_production::execute_build, start_production::execute_serve},
    tests::execute::{execute_coverage, execute_tests},
    utils::terminal::{help, step},
};

use super::{
    cmd_list::{check_for_cli_cmds, CliCmds},
    interactive::{start_interactive, InquireUserInput, RealCommandExecutor},
};

/// Everything a cli command can do, so the command dispatch can be exercised
/// without running the real commands.
pub trait CmdActions {
    fn help(&self);
    fn sync_git_hooks(&self);
    fn remove_git_hooks(&self);
    fn create_toml(&self);
    fn interactive(&self);
    fn system_check(&self);
    fn build(&self);
    fn test(&self);
    fn serve(&self);
    fn coverage(&self);
}

pub struct RealCmdActions;

impl CmdActions for RealCmdActions {
    fn help(&self) {
        help();
    }

    fn sync_git_hooks(&self) {
        // Copy the git hooks to the .git/hooks folder
        // Enjoy pre-commit, pre-push and commit-msg hooks that will help you to maintain the code quality
        step("Syncing the git hooks");
        copy_git_hooks();
    }

    fn remove_git_hooks(&self) {
        // Remove the git hooks from the .git/hooks folder
        // This will remove the pre-commit, pre-push and commit-msg hooks
        step("Removing the git hooks");
        remove_git_hooks();
    }

    fn create_toml(&self) {
        create_toml_file(ASTROX_TOML.to_string()).expect("Failed to create Astrox.toml file");
    }

    fn interactive(&self) {
        start_interactive(&InquireUserInput, &RealCommandExecutor);
    }

    fn system_check(&self) {
        run_system_checks("dev");
    }

    fn build(&self) {
        step("Building the project");
        execute_build();
    }

    fn test(&self) {
        step("Testing the project");
        execute_tests();
    }

    fn serve(&self) {
        step("Serving the project");
        execute_serve();
    }

    fn coverage(&self) {
        step("Running rust the coverage");
        execute_coverage();
    }
}

/// What the cli should do once the command has been handled.
#[derive(Debug, PartialEq, Eq)]
pub enum CmdOutcome {
    /// The command is done, the cli exits.
    Exit,
    /// The cli carries on and starts a server.
    Continue,
}

/// Run the action behind `cmd` and report whether the cli is done.
/// `Run` and `Interactive` are the only commands that let the cli carry on.
pub fn dispatch_cmd(cmd: CliCmds, actions: &dyn CmdActions) -> CmdOutcome {
    match cmd {
        CliCmds::Run => CmdOutcome::Continue,
        CliCmds::Interactive => {
            actions.interactive();
            CmdOutcome::Continue
        }
        CliCmds::Help => {
            actions.help();
            CmdOutcome::Exit
        }
        CliCmds::SyncGitHooks => {
            actions.sync_git_hooks();
            CmdOutcome::Exit
        }
        CliCmds::RemoveGitHooks => {
            actions.remove_git_hooks();
            CmdOutcome::Exit
        }
        CliCmds::CreateToml => {
            actions.create_toml();
            CmdOutcome::Exit
        }
        CliCmds::SystemCheck => {
            actions.system_check();
            CmdOutcome::Exit
        }
        CliCmds::Build => {
            actions.build();
            CmdOutcome::Exit
        }
        CliCmds::Test => {
            actions.test();
            CmdOutcome::Exit
        }
        CliCmds::Serve => {
            actions.serve();
            CmdOutcome::Exit
        }
        CliCmds::Coverage => {
            actions.coverage();
            CmdOutcome::Exit
        }
    }
}

pub fn execute_cmd(args: &Vec<String>) {
    let cmd = check_for_cli_cmds(args);

    if dispatch_cmd(cmd, &RealCmdActions) == CmdOutcome::Exit {
        std::process::exit(0);
    }
}
