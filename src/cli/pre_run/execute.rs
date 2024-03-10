use super::{
    system_checks::run_system_checks,
    utils::{first_run, git_hooks},
};

pub fn execute(env: &str) {
    if first_run::is_first_run() && env == "dev" {
        git_hooks::copy_git_hooks();
        run_system_checks(env);
    }

    if env == "prod" {
        run_system_checks(env);
    }
}
