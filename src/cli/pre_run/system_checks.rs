use crate::cli::utils::terminal::{error, spacer, step, success};

use super::{
    cargo::validate::{validate_cargo_watch, validate_commitlint_rs, validate_rustc_version},
    npm::validate::{validate_frontend_project, validate_node},
};

/// Run system checks before starting the development or production server
/// The system checks will check if the required tools are installed
/// The system checks will also check if the required projects are installed

pub fn run_system_checks(env: &str) {
    spacer();
    step("Running system checks ...");

    match env {
        "dev" => {
            validate_rustc_version();
            validate_cargo_watch();
            validate_commitlint_rs();
            validate_node();
            validate_frontend_project();
        }
        "prod" => {
            validate_rustc_version();
            validate_node();
            validate_frontend_project();
        }
        _ => {
            error("Invalid environment, about to panic");
            panic!();
        }
    }

    success("System checks passed successfully");
}
