mod cli;
use crate::cli::config::get_config::get_config;
use crate::cli::development::start_development::start_development;
use crate::cli::pre_run::system_checks::run_system_checks;
use crate::cli::production::start_production::start_production;
use crate::cli::utils::terminal::do_splash;
use cli::cmds::execute_cmd::execute_cmd;
use std::env;

/// The main Astro X cli
/// The cli is responsible for starting the development and production servers

fn main() {
    // Print the splash screen
    do_splash();

    // Get the arguments from the command line
    let args = env::args().collect::<Vec<String>>();

    execute_cmd(&args);

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
