use std::cell::RefCell;

use crate::cli::cmds::execute_cmd::CmdActions;

/// Records which action the dispatch asked for instead of running it.
struct SpyCmdActions {
    calls: RefCell<Vec<String>>,
}

impl SpyCmdActions {
    fn new() -> Self {
        Self {
            calls: RefCell::new(vec![]),
        }
    }

    fn record(&self, name: &str) {
        self.calls.borrow_mut().push(name.to_string());
    }

    fn calls(&self) -> Vec<String> {
        self.calls.borrow().clone()
    }
}

impl CmdActions for SpyCmdActions {
    fn help(&self) {
        self.record("help");
    }
    fn sync_git_hooks(&self) {
        self.record("sync_git_hooks");
    }
    fn remove_git_hooks(&self) {
        self.record("remove_git_hooks");
    }
    fn create_toml(&self) {
        self.record("create_toml");
    }
    fn interactive(&self) {
        self.record("interactive");
    }
    fn system_check(&self) {
        self.record("system_check");
    }
    fn build(&self) {
        self.record("build");
    }
    fn test(&self) {
        self.record("test");
    }
    fn serve(&self) {
        self.record("serve");
    }
    fn coverage(&self) {
        self.record("coverage");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::cmds::cmd_list::check_for_cli_cmds;
    use crate::cli::cmds::execute_cmd::{dispatch_cmd, CmdOutcome, RealCmdActions};

    /// Run the whole path a real cli invocation takes, minus the process exit.
    fn dispatch_args(args: &[&str]) -> (Vec<String>, CmdOutcome) {
        let args: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
        let actions = SpyCmdActions::new();
        let outcome = dispatch_cmd(check_for_cli_cmds(&args), &actions);

        (actions.calls(), outcome)
    }

    #[test]
    fn test_execute_cmd_system_check() {
        let (calls, outcome) = dispatch_args(&["--system-check"]);

        assert_eq!(calls, vec!["system_check"]);
        assert_eq!(outcome, CmdOutcome::Exit);
    }

    #[test]
    fn test_execute_cmd_help() {
        let (calls, outcome) = dispatch_args(&["--help"]);

        assert_eq!(calls, vec!["help"]);
        assert_eq!(outcome, CmdOutcome::Exit);
    }

    #[test]
    fn test_execute_cmd_sync_git_hooks() {
        let (calls, outcome) = dispatch_args(&["--sync-git-hooks"]);

        assert_eq!(calls, vec!["sync_git_hooks"]);
        assert_eq!(outcome, CmdOutcome::Exit);
    }

    #[test]
    fn test_execute_cmd_remove_git_hooks() {
        let (calls, outcome) = dispatch_args(&["--remove-git-hooks"]);

        assert_eq!(calls, vec!["remove_git_hooks"]);
        assert_eq!(outcome, CmdOutcome::Exit);
    }

    #[test]
    fn test_execute_cmd_create_toml() {
        let (calls, outcome) = dispatch_args(&["--create-toml"]);

        assert_eq!(calls, vec!["create_toml"]);
        assert_eq!(outcome, CmdOutcome::Exit);
    }

    #[test]
    fn test_execute_cmd_build() {
        let (calls, outcome) = dispatch_args(&["--build"]);

        assert_eq!(calls, vec!["build"]);
        assert_eq!(outcome, CmdOutcome::Exit);
    }

    #[test]
    fn test_execute_cmd_test() {
        let (calls, outcome) = dispatch_args(&["--test"]);

        assert_eq!(calls, vec!["test"]);
        assert_eq!(outcome, CmdOutcome::Exit);
    }

    #[test]
    fn test_execute_cmd_serve() {
        let (calls, outcome) = dispatch_args(&["--serve"]);

        assert_eq!(calls, vec!["serve"]);
        assert_eq!(outcome, CmdOutcome::Exit);
    }

    #[test]
    fn test_execute_cmd_coverage() {
        let (calls, outcome) = dispatch_args(&["--coverage"]);

        assert_eq!(calls, vec!["coverage"]);
        assert_eq!(outcome, CmdOutcome::Exit);
    }

    #[test]
    fn test_execute_cmd_interactive_lets_the_cli_carry_on() {
        // Interactive dispatches the selected command itself, so the cli must
        // not exit on its own after it
        let (calls, outcome) = dispatch_args(&["--interactive"]);

        assert_eq!(calls, vec!["interactive"]);
        assert_eq!(outcome, CmdOutcome::Continue);
    }

    #[test]
    fn test_execute_cmd_run_does_nothing_and_carries_on() {
        let (calls, outcome) = dispatch_args(&["--run"]);

        assert!(calls.is_empty());
        assert_eq!(outcome, CmdOutcome::Continue);
    }

    #[test]
    fn test_real_actions_print_the_help() {
        // The help and the system checks are the two real actions that have no
        // side effects, so the wiring to them can be checked for real
        RealCmdActions.help();
    }

    #[test]
    fn test_real_actions_run_the_system_checks() {
        RealCmdActions.system_check();
    }

    #[test]
    fn test_execute_cmd_without_a_known_command_carries_on() {
        // The binary path is always argv[0], it must not trigger anything
        let (calls, outcome) = dispatch_args(&["target/debug/astro_x_runner"]);

        assert!(calls.is_empty());
        assert_eq!(outcome, CmdOutcome::Continue);
    }
}
