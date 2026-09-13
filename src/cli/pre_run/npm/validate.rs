use crate::cli::pre_run::utils::install::{
    confirm_and_install, Confirmation, InquireConfirmation, InstallCommand, Installer,
    RealInstaller, ValidationOutcome,
};
use crate::cli::utils::terminal::{error, success};

use super::checks::{is_frontend_project_installed, is_node_installed, NPM};

/// Report on the node version. Node cannot be installed for the user, so a
/// missing or too old node is simply fatal.
pub fn validate_node_with(installed: bool) -> ValidationOutcome {
    if installed {
        success("node is installed and its version is higher than 20.9.0");
        return ValidationOutcome::AlreadyInstalled;
    }

    error("node is not installed, or its version is below 20.9.0 please install it and try again. Panicking...");

    ValidationOutcome::Unsatisfiable
}

pub fn validate_node() {
    let outcome = validate_node_with(is_node_installed());

    if outcome.is_fatal() {
        panic!()
    }
}

/// Validate the astro frontend project, offering to install it when its
/// dependencies are missing.
pub fn validate_frontend_project_with(
    installed: bool,
    confirmation: &dyn Confirmation,
    installer: &dyn Installer,
) -> ValidationOutcome {
    if installed {
        success("astro framework is installed");
        return ValidationOutcome::AlreadyInstalled;
    }

    error("Astro framework is not installed");

    let outcome = confirm_and_install(
        confirmation,
        installer,
        "Do you want to install astro framework ?",
        None,
        "Installing the astro framework ...",
        &InstallCommand {
            program: NPM,
            args: &["install"],
            dir: Some("./src/frontend"),
            failure_message: "Failed to install the frontend project",
        },
    );

    if outcome == ValidationOutcome::Installed {
        success("Astro framework installed successfully")
    }

    outcome
}

pub fn validate_frontend_project() {
    let outcome = validate_frontend_project_with(
        is_frontend_project_installed(),
        &InquireConfirmation,
        &RealInstaller,
    );

    if outcome.is_fatal() {
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::pre_run::utils::install::test_doubles::{SpyInstaller, StubConfirmation};

    #[test]
    fn test_validate_node_installed() {
        // Test when node is installed and version is above required version
        assert_eq!(is_node_installed(), true);

        let outcome = validate_node_with(true);

        assert_eq!(outcome, ValidationOutcome::AlreadyInstalled);
        assert!(!outcome.is_fatal());
    }

    #[test]
    fn test_validate_node_missing_is_fatal() {
        let outcome = validate_node_with(false);

        assert_eq!(outcome, ValidationOutcome::Unsatisfiable);
        assert!(outcome.is_fatal());
    }

    #[test]
    fn test_validate_frontend_project_installed() {
        // The astro project of this repository is installed
        assert_eq!(is_frontend_project_installed(), true);

        let confirmation = StubConfirmation::answering(Ok(true));
        let installer = SpyInstaller::new();

        let outcome = validate_frontend_project_with(true, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::AlreadyInstalled);
        // Nothing was asked and nothing was installed
        assert!(confirmation.asked.borrow().is_empty());
        assert!(installer.commands().is_empty());
    }

    #[test]
    fn test_validate_frontend_project_missing_runs_npm_install_in_the_frontend() {
        let confirmation = StubConfirmation::answering(Ok(true));
        let installer = SpyInstaller::new();

        let outcome = validate_frontend_project_with(false, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::Installed);
        assert!(!outcome.is_fatal());
        assert_eq!(
            installer.commands(),
            vec![format!("{} install @ ./src/frontend", NPM)]
        );
        assert_eq!(
            *confirmation.asked.borrow(),
            vec!["Do you want to install astro framework ?".to_string()]
        );
    }

    #[test]
    fn test_validate_frontend_project_missing_and_refused_is_fatal() {
        let confirmation = StubConfirmation::answering(Ok(false));
        let installer = SpyInstaller::new();

        let outcome = validate_frontend_project_with(false, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::Declined);
        assert!(outcome.is_fatal());
        assert!(installer.commands().is_empty());
    }

    #[test]
    fn test_validate_frontend_project_missing_with_a_broken_prompt_is_fatal() {
        let confirmation = StubConfirmation::answering(Err(()));
        let installer = SpyInstaller::new();

        let outcome = validate_frontend_project_with(false, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::PromptFailed);
        assert!(outcome.is_fatal());
        assert!(installer.commands().is_empty());
    }
}
