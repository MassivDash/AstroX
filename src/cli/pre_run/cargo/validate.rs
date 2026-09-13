use crate::cli::pre_run::cargo::checks::{
    is_cargo_watch_installed, is_commitlint_rs_installed, is_rustc_higher_than_required,
    REQUIRED_VERSION,
};
use crate::cli::pre_run::utils::install::{
    confirm_and_install, Confirmation, InquireConfirmation, InstallCommand, Installer,
    RealInstaller, ValidationOutcome,
};
use crate::cli::utils::terminal::{error, spacer, success};

use super::checks::is_llvm_cov_installed;

/// Validate cargo-watch, offering to install it when it is missing.
pub fn validate_cargo_watch_with(
    installed: bool,
    confirmation: &dyn Confirmation,
    installer: &dyn Installer,
) -> ValidationOutcome {
    if installed {
        success("cargo-watch is installed");
        return ValidationOutcome::AlreadyInstalled;
    }

    error("cargo-watch is not installed");
    spacer();

    confirm_and_install(
        confirmation,
        installer,
        "Do you want to install cargo-watch ?",
        Some("cargo-watch must be installed globally in order to spy on changes to the server"),
        "Installing cargo-watch ...",
        &InstallCommand {
            program: "cargo",
            args: &["install", "cargo-watch"],
            dir: None,
            failure_message: "Failed to install cargo-watch",
        },
    )
}

pub fn validate_cargo_watch() {
    let outcome = validate_cargo_watch_with(
        is_cargo_watch_installed(),
        &InquireConfirmation,
        &RealInstaller,
    );

    if outcome.is_fatal() {
        panic!();
    }
}

/// Validate commitlint-rs, offering to install it when it is missing.
pub fn validate_commitlint_rs_with(
    installed: bool,
    confirmation: &dyn Confirmation,
    installer: &dyn Installer,
) -> ValidationOutcome {
    if installed {
        success("commitlint-rs is installed");
        return ValidationOutcome::AlreadyInstalled;
    }

    error("commitlint-rs is not installed");
    spacer();

    confirm_and_install(
        confirmation,
        installer,
        "Do you want to install commitlint-rs ?",
        Some("commitlint-rs must be installed globally in order to lint the commit messages, this is the recommended way to go"),
        "Installing commitlint-rs ...",
        &InstallCommand {
            program: "cargo",
            args: &["install", "commitlint-rs"],
            dir: None,
            failure_message: "Failed to install commitlint-rs",
        },
    )
}

pub fn validate_commitlint_rs() {
    let outcome = validate_commitlint_rs_with(
        is_commitlint_rs_installed(),
        &InquireConfirmation,
        &RealInstaller,
    );

    if outcome.is_fatal() {
        panic!();
    }
}

/// Validate cargo-llvm-cov, offering to install it when it is missing.
pub fn validate_llcov_with(
    installed: bool,
    confirmation: &dyn Confirmation,
    installer: &dyn Installer,
) -> ValidationOutcome {
    if installed {
        success("llvm-cov is installed");
        return ValidationOutcome::AlreadyInstalled;
    }

    error("llvm-cov is not installed");
    spacer();

    confirm_and_install(
        confirmation,
        installer,
        "Do you want to install llvm-cov for code coverage reporting ?",
        Some("llvm-cov must be installed globally in order to produce rust coverage report, this is the recommended way to go"),
        "Installing llvm-cov ...",
        &InstallCommand {
            program: "cargo",
            args: &["install", "cargo-llvm-cov"],
            dir: None,
            failure_message: "Failed to install llvm-cov",
        },
    )
}

pub fn validate_llcov() {
    let outcome = validate_llcov_with(
        is_llvm_cov_installed(),
        &InquireConfirmation,
        &RealInstaller,
    );

    if outcome.is_fatal() {
        panic!();
    }
}

/// Report on the rustc version. There is nothing we could install for the user
/// here, so a version below the requirement is simply fatal.
pub fn validate_rustc_version_with(high_enough: bool) -> ValidationOutcome {
    if high_enough {
        success("Rustc version is higher than required");
        return ValidationOutcome::AlreadyInstalled;
    }

    error("Rustc version is lower than required ");
    spacer();
    error(format!("Rustc version must be higher than {}", REQUIRED_VERSION).as_str());

    ValidationOutcome::Unsatisfiable
}

pub fn validate_rustc_version() {
    let outcome = validate_rustc_version_with(is_rustc_higher_than_required());

    if outcome.is_fatal() {
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::pre_run::utils::install::test_doubles::{SpyInstaller, StubConfirmation};

    #[test]
    fn test_validate_cargo_watch_installed() {
        // Test when cargo-watch is installed
        assert_eq!(is_cargo_watch_installed(), true);

        let confirmation = StubConfirmation::answering(Ok(true));
        let installer = SpyInstaller::new();

        let outcome = validate_cargo_watch_with(true, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::AlreadyInstalled);
        // Nothing was asked and nothing was installed
        assert!(confirmation.asked.borrow().is_empty());
        assert!(installer.commands().is_empty());
    }

    #[test]
    fn test_validate_cargo_watch_missing_and_accepted() {
        let confirmation = StubConfirmation::answering(Ok(true));
        let installer = SpyInstaller::new();

        let outcome = validate_cargo_watch_with(false, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::Installed);
        assert!(!outcome.is_fatal());
        assert_eq!(
            installer.commands(),
            vec!["cargo install cargo-watch @ .".to_string()]
        );
    }

    #[test]
    fn test_validate_cargo_watch_missing_and_refused_is_fatal() {
        let confirmation = StubConfirmation::answering(Ok(false));
        let installer = SpyInstaller::new();

        let outcome = validate_cargo_watch_with(false, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::Declined);
        assert!(outcome.is_fatal());
        assert!(installer.commands().is_empty());
    }

    #[test]
    fn test_validate_commitlint_rs_installed() {
        // Test when commitlint-rs is installed
        assert_eq!(is_commitlint_rs_installed(), true);

        let confirmation = StubConfirmation::answering(Ok(true));
        let installer = SpyInstaller::new();

        let outcome = validate_commitlint_rs_with(true, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::AlreadyInstalled);
        assert!(installer.commands().is_empty());
    }

    #[test]
    fn test_validate_commitlint_rs_missing_and_accepted() {
        let confirmation = StubConfirmation::answering(Ok(true));
        let installer = SpyInstaller::new();

        let outcome = validate_commitlint_rs_with(false, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::Installed);
        assert_eq!(
            installer.commands(),
            vec!["cargo install commitlint-rs @ .".to_string()]
        );
    }

    #[test]
    fn test_validate_commitlint_rs_missing_with_a_broken_prompt_is_fatal() {
        let confirmation = StubConfirmation::answering(Err(()));
        let installer = SpyInstaller::new();

        let outcome = validate_commitlint_rs_with(false, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::PromptFailed);
        assert!(outcome.is_fatal());
        assert!(installer.commands().is_empty());
    }

    #[test]
    fn test_validate_llvm_cov_installed() {
        // Test when llvm-cov is installed
        assert_eq!(is_llvm_cov_installed(), true);

        let confirmation = StubConfirmation::answering(Ok(true));
        let installer = SpyInstaller::new();

        let outcome = validate_llcov_with(true, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::AlreadyInstalled);
        assert!(installer.commands().is_empty());
    }

    #[test]
    fn test_validate_llvm_cov_missing_installs_the_cargo_llvm_cov_crate() {
        let confirmation = StubConfirmation::answering(Ok(true));
        let installer = SpyInstaller::new();

        let outcome = validate_llcov_with(false, &confirmation, &installer);

        assert_eq!(outcome, ValidationOutcome::Installed);
        assert_eq!(
            installer.commands(),
            vec!["cargo install cargo-llvm-cov @ .".to_string()]
        );
    }

    #[test]
    fn test_validate_llvm_cov_missing_and_refused_is_fatal() {
        let confirmation = StubConfirmation::answering(Ok(false));
        let installer = SpyInstaller::new();

        assert!(validate_llcov_with(false, &confirmation, &installer).is_fatal());
    }

    #[test]
    fn test_validate_rustc_version_higher() {
        // Test when rustc version is higher than required
        assert_eq!(is_rustc_higher_than_required(), true);

        let outcome = validate_rustc_version_with(true);

        assert_eq!(outcome, ValidationOutcome::AlreadyInstalled);
        assert!(!outcome.is_fatal());
    }

    #[test]
    fn test_validate_rustc_version_too_low_is_fatal() {
        let outcome = validate_rustc_version_with(false);

        assert_eq!(outcome, ValidationOutcome::Unsatisfiable);
        assert!(outcome.is_fatal());
    }
}
