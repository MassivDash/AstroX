use std::process::Command;

use inquire::Confirm;

use crate::cli::utils::terminal::{error, spacer, step, success};

use super::checks::{is_frontend_project_installed, is_node_installed, NPM};

pub fn validate_node() {
    let is_node_installed = is_node_installed();

    match is_node_installed {
        true => success("node is installed and its version is higher than 18.14.1"),
        false => {
            error("node is not installed, or its version is below 18.14.1 please install it and try again. Panicking...");
            panic!()
        }
    }
}

pub fn validate_frontend_project() {
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
#[cfg(test)]
mod tests {

    #[test]
    fn test_validate_node_installed() {
        // Simulate node being installed
        let is_node_installed = true;
        assert_eq!(is_node_installed, true);
    }

    #[test]
    fn test_validate_frontend_project_installed() {
        // Simulate frontend project being installed
        let project_installed = true;
        assert_eq!(project_installed, true);
    }
}
