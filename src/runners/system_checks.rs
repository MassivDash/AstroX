use super::terminal::{error, spacer, step, success};
use inquire::Confirm;
use std::process::Command;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &str = "npm";

pub fn is_cargo_watch_installed() -> bool {
    let output = Command::new("cargo")
        .arg("install")
        .arg("--list")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    output_str.contains("cargo-watch")
}

// check if node is installed
// check if the node version is above 18.14.1
// versions below will panic on astro commands
pub fn is_node_installed() -> bool {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut version = output_str.trim();

    // remove the v from v20.10.0
    if version.starts_with('v') {
        version = &version[1..];
    }

    let version_split: Vec<&str> = version.split('.').collect();
    if version_split[0].parse::<i32>().unwrap() > 18 {
        return true;
    };
    if version_split[0].parse::<i32>().unwrap() == 18
        && version_split[1].parse::<i32>().unwrap() > 14
    {
        return true;
    };
    if version_split[0].parse::<i32>().unwrap() == 18
        && version_split[1].parse::<i32>().unwrap() == 14
        && version_split[2].parse::<i32>().unwrap() >= 1
    {
        return true;
    };
    false
}

pub fn is_frontend_project_installed() -> bool {
    let project = std::path::Path::new("./src/frontend/package.json").exists();
    let installed = std::path::Path::new("./src/frontend/node_modules").exists();
    project && installed
}

pub fn run_system_checks(prod_astro_build: bool) {
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

    // Check if the user has node installed, panic and inform the user what to do

    let is_node_installed = is_node_installed();

    match is_node_installed {
        true => success("node is installed and its version is higher than 18.14.1"),
        false => {
            error("node is not installed, or its version is below 18.14.1 please install it and try again. Panicking...");
            panic!()
        }
    }

    if prod_astro_build {
        let project = is_frontend_project_installed();

        match project {
            true => success("astro framework is installed"),
            false => {
                error("Astro framework is not installed");
                let ans = Confirm::new("Do you want to install astro framework ?")
                    .with_default(false)
                    .prompt();

                match ans {
                    Ok(true) => {
                        spacer();
                        step("Installing the astro framework ...");
                        Command::new(NPM)
                            .arg("install")
                            .current_dir("./src/frontend")
                            .spawn()
                            .expect("Failed to install the frontend project")
                            .wait()
                            .expect("Failed to install the frontend project");

                        spacer();
                        success("Astro framework installed successfully")
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cargo_watch_installed() {
        // Test when cargo-watch is installed
        assert_eq!(is_cargo_watch_installed(), true);
    }

    #[test]
    fn test_is_node_installed() {
        // Test when node is installed with a version higher than 18.14.1
        assert_eq!(is_node_installed(), true);
    }

    #[test]
    fn test_is_frontend_project_installed() {
        // Test when the frontend project is installed
        assert_eq!(is_frontend_project_installed(), true);
    }
}
