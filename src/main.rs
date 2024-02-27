mod cli;
use cli::utils::terminal::help;

use crate::cli::config::collect_args::{check_for_cli_cmds, CliCmds};
use crate::cli::config::get_config::get_config;
use crate::cli::config::toml::create_toml_file;
use crate::cli::development::start_development::start_development;
use crate::cli::pre_run::git_hooks::copy_git_hooks;
use crate::cli::pre_run::system_checks::run_system_checks;
use crate::cli::production::start_production::start_production;
use crate::cli::utils::terminal::{do_splash, step};
use std::env;

/// The main Astro X cli
/// The cli is responsible for starting the development and production servers

fn main() {
    // Print the splash screen
    do_splash();

    // Get the arguments from the command line
    let args = env::args().collect::<Vec<String>>();

    let cli_cmd = check_for_cli_cmds(&args);

    if cli_cmd != CliCmds::Run {
        match cli_cmd {
            CliCmds::Help => {
                return help();
            }
            CliCmds::SyncGitHooks => {
                // Copy the git hooks to the .git/hooks folder
                // Enjoy pre-commit, pre-push and commit-msg hooks that will help you to maintain the code quality
                step("Syncing the git hooks");
                return copy_git_hooks();
            }
            CliCmds::CreateToml => {
                create_toml_file("Astrox.toml".to_string())
                    .expect("Failed to create Astrox.toml file");
                return;
            }
            CliCmds::Run => {}
        }
        return;
    }

    // Create config
    let config = get_config(&args);

    // Run the system checks
    run_system_checks(&config.env, config.prod_astro_build);

    if config.env == "dev" {
        // Start the development server
        start_development(config);
    } else {
        // Start the production server
        start_production(config);
    }
}
