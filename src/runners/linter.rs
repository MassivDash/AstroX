use super::system_checks::NPM;
use super::terminal::step;

use std::process::Command;

pub fn start_npm_linting() {
    let node_watch = Command::new(NPM)
        .arg("run")
        .arg("lint")
        .current_dir("./src/frontend")
        .spawn()
        .expect("Failed to start npm linting")
        .wait()
        .expect("Failed to start npm linting");

    match node_watch.success() {
        true => step("NPM linting done successfully"),
        false => panic!("Linting failed, fix code"),
    }
}
