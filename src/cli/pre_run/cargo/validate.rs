use crate::cli::pre_run::cargo::checks::{
    is_cargo_watch_installed, is_commitlint_rs_installed, is_rustc_higher_than_required,
    REQUIRED_VERSION,
};
use crate::cli::utils::terminal::{error, spacer, step, success};
use inquire::Confirm;
use std::process::Command;

pub fn validate_cargo_watch() {
    let is_cargo_watch_installed = is_cargo_watch_installed();

    match is_cargo_watch_installed {
        true => success("cargo-watch is installed"),
        false => {
            error("cargo-watch is not installed");
            spacer();
            let ans = Confirm::new("Do you want to install cargo-watch ?")
            .with_default(false)
            .with_help_message("cargo-watch must be installed globally in order to spy on changes to the server")
            .prompt();

            match ans {
                Ok(true) => {
                    spacer();
                    step("Installing cargo-watch ...");
                    Command::new("cargo")
                        .arg("install")
                        .arg("cargo-watch")
                        .spawn()
                        .expect("Failed to install cargo-watch")
                        .wait()
                        .expect("Failed to install cargo-watch");
                    spacer();
                }
                Ok(false) => {
                    error("That's too bad, we have to quit now");
                    panic!();
                }
                Err(_) => {
                    error("Error with prompt, about to panic");
                    panic!();
                }
            }
        }
    }
}

pub fn validate_commitlint_rs() {
    let is_commitlint_rs_installed = is_commitlint_rs_installed();

    match is_commitlint_rs_installed {
        true => success("commitlint-rs is installed"),
        false => {
            error("commitlint-rs is not installed");
            spacer();
            let ans = Confirm::new("Do you want to install commitlint-rs ?")
            .with_default(false)
            .with_help_message("commitlint-rs must be installed globally in order to lint the commit messages, this is the recommended way to go")
            .prompt();

            match ans {
                Ok(true) => {
                    spacer();
                    step("Installing commitlint-rs ...");
                    Command::new("cargo")
                        .arg("install")
                        .arg("commitlint-rs")
                        .spawn()
                        .expect("Failed to install commitlint-rs")
                        .wait()
                        .expect("Failed to install commitlint-rs");
                    spacer();
                }
                Ok(false) => {
                    error("That's too bad, we have to quit now");
                    panic!();
                }
                Err(_) => {
                    error("Error with prompt, about to panic");
                    panic!();
                }
            }
        }
    }
}

pub fn validate_rustc_version() {
    let is_rustc_higher_than_required = is_rustc_higher_than_required();

    match is_rustc_higher_than_required {
        true => success("Rustc version is higher than required"),
        false => {
            error("Rustc version is lower than required ");
            spacer();
            error(format!("Rustc version must be higher than {}", REQUIRED_VERSION).as_str());
            panic!();
        }
    }
}
