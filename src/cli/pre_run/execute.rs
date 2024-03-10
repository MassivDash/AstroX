use super::{system_checks::run_system_checks, utils::git_hooks};

pub fn execute(env: &str) {
    if !git_hooks::check_if_git_hooks_are_installed() {
        git_hooks::copy_git_hooks();
    }

    run_system_checks(env);
}
