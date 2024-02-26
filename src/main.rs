mod cli;
use crate::cli::development::start_development::start_development;
use crate::cli::pre_run::git_hooks::copy_git_hooks;
use crate::cli::pre_run::system_checks::run_system_checks;
use crate::cli::production::start_production::start_production;
use crate::cli::utils::terminal::{do_splash, step};
use cli::args::collect_args::collect_args;

/// The main Astro X cli
/// The cli is responsible for starting the development and production servers

fn main() {
    // Print the splash screen
    do_splash();

    // Get the cmd line arguments
    let args = collect_args();

    // Run the system checks

    run_system_checks(&args.env, args.prod_astro_build);

    if args.env == "dev" {
        // During development is could be possible that user starts the project for the first time, or has altered the git hooks
        // Copy the git hooks to the .git/hooks folder
        // Enjoy pre-commit, pre-push and commit-msg hooks that will help you to maintain the code quality
        step("Syncing the git hooks");
        copy_git_hooks();

        // Start the development server
        start_development(args.host, args.port, args.astro_port);
    } else {
        // Start the production server
        start_production(args.host, args.port, args.prod_astro_build)
    }
}
