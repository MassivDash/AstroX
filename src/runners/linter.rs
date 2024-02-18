use super::system_checks::NPM;
use super::terminal::{error, spacer, step, warning};

use std::process::Command;

pub fn start_npm_linting() {
    warning("Running cargo runner bride");

    let node_watch = Command::new(NPM)
        .arg("run")
        .arg("lint")
        .current_dir("./src/frontend")
        .spawn()
        .expect("Failed to start npm linting")
        .wait()
        .expect("Failed to start npm linting");

    match node_watch.success() {
        true => {
            spacer();
            step("NPM linting done successfully")
        }
        false => {
            error("NPM linting failed");
            panic!("Linting failed, fix code")
        }
    }
}
