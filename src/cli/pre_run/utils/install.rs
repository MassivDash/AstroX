use crate::cli::utils::terminal::{error, spacer, step};
use inquire::Confirm;
use std::process::Command;

/// Asks the user a yes / no question.
pub trait Confirmation {
    /// `Err(())` means the prompt itself failed, for example when there is no terminal.
    fn confirm<'a>(&self, message: &'a str, help_message: Option<&'a str>) -> Result<bool, ()>;
}

pub struct InquireConfirmation;

impl Confirmation for InquireConfirmation {
    fn confirm<'a>(&self, message: &'a str, help_message: Option<&'a str>) -> Result<bool, ()> {
        let prompt = Confirm::new(message).with_default(false);

        let prompt = match help_message {
            Some(help_message) => prompt.with_help_message(help_message),
            None => prompt,
        };

        prompt.prompt().map_err(|_| ())
    }
}

/// The command that installs a missing tool.
pub struct InstallCommand<'a> {
    pub program: &'a str,
    pub args: &'a [&'a str],
    pub dir: Option<&'a str>,
    pub failure_message: &'a str,
}

/// Runs the install command of a missing tool.
pub trait Installer {
    fn install(&self, command: &InstallCommand);
}

pub struct RealInstaller;

impl Installer for RealInstaller {
    fn install(&self, command: &InstallCommand) {
        let mut process = Command::new(command.program);
        process.args(command.args);

        if let Some(dir) = command.dir {
            process.current_dir(dir);
        }

        process
            .spawn()
            .expect(command.failure_message)
            .wait()
            .expect(command.failure_message);
    }
}

/// The result of validating one required tool.
#[derive(Debug, PartialEq, Eq)]
pub enum ValidationOutcome {
    /// The tool was already there, nothing to do.
    AlreadyInstalled,
    /// The tool was missing and has just been installed.
    Installed,
    /// The tool was missing and the user refused to install it.
    Declined,
    /// The tool was missing and we could not even ask the user.
    PromptFailed,
    /// The requirement is not met and there is nothing we could install to fix it.
    Unsatisfiable,
}

impl ValidationOutcome {
    /// The cli cannot run without the tool, so the caller has to give up.
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            ValidationOutcome::Declined
                | ValidationOutcome::PromptFailed
                | ValidationOutcome::Unsatisfiable
        )
    }
}

/// Ask the user whether a missing tool should be installed and act on the answer.
pub fn confirm_and_install(
    confirmation: &dyn Confirmation,
    installer: &dyn Installer,
    message: &str,
    help_message: Option<&str>,
    installing_message: &str,
    command: &InstallCommand,
) -> ValidationOutcome {
    match confirmation.confirm(message, help_message) {
        Ok(true) => {
            spacer();
            step(installing_message);
            installer.install(command);
            spacer();
            ValidationOutcome::Installed
        }
        Ok(false) => {
            error("That's too bad, we have to quit now");
            ValidationOutcome::Declined
        }
        Err(_) => {
            error("Error with prompt, about to panic");
            ValidationOutcome::PromptFailed
        }
    }
}

#[cfg(test)]
pub mod test_doubles {
    use super::*;
    use std::cell::RefCell;

    /// A confirmation prompt that answers whatever the test asked for.
    pub struct StubConfirmation {
        pub answer: Result<bool, ()>,
        pub asked: RefCell<Vec<String>>,
    }

    impl StubConfirmation {
        pub fn answering(answer: Result<bool, ()>) -> Self {
            Self {
                answer,
                asked: RefCell::new(vec![]),
            }
        }
    }

    impl Confirmation for StubConfirmation {
        fn confirm<'a>(
            &self,
            message: &'a str,
            _help_message: Option<&'a str>,
        ) -> Result<bool, ()> {
            self.asked.borrow_mut().push(message.to_string());
            self.answer
        }
    }

    /// An installer that records the command instead of running it.
    pub struct SpyInstaller {
        pub installed: RefCell<Vec<String>>,
    }

    impl SpyInstaller {
        pub fn new() -> Self {
            Self {
                installed: RefCell::new(vec![]),
            }
        }

        pub fn commands(&self) -> Vec<String> {
            self.installed.borrow().clone()
        }
    }

    impl Installer for SpyInstaller {
        fn install(&self, command: &InstallCommand) {
            self.installed.borrow_mut().push(format!(
                "{} {} @ {}",
                command.program,
                command.args.join(" "),
                command.dir.unwrap_or(".")
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_doubles::{SpyInstaller, StubConfirmation};
    use super::*;

    fn install_command() -> InstallCommand<'static> {
        InstallCommand {
            program: "cargo",
            args: &["install", "cargo-watch"],
            dir: None,
            failure_message: "Failed to install cargo-watch",
        }
    }

    #[test]
    fn test_confirm_and_install_runs_the_install_command_on_yes() {
        let confirmation = StubConfirmation::answering(Ok(true));
        let installer = SpyInstaller::new();

        let outcome = confirm_and_install(
            &confirmation,
            &installer,
            "Do you want to install cargo-watch ?",
            Some("some help"),
            "Installing cargo-watch ...",
            &install_command(),
        );

        assert_eq!(outcome, ValidationOutcome::Installed);
        assert_eq!(
            installer.commands(),
            vec!["cargo install cargo-watch @ .".to_string()]
        );
        assert_eq!(
            *confirmation.asked.borrow(),
            vec!["Do you want to install cargo-watch ?".to_string()]
        );
    }

    #[test]
    fn test_confirm_and_install_installs_nothing_on_no() {
        let confirmation = StubConfirmation::answering(Ok(false));
        let installer = SpyInstaller::new();

        let outcome = confirm_and_install(
            &confirmation,
            &installer,
            "Do you want to install cargo-watch ?",
            None,
            "Installing cargo-watch ...",
            &install_command(),
        );

        assert_eq!(outcome, ValidationOutcome::Declined);
        assert!(installer.commands().is_empty());
    }

    #[test]
    fn test_confirm_and_install_reports_a_broken_prompt() {
        let confirmation = StubConfirmation::answering(Err(()));
        let installer = SpyInstaller::new();

        let outcome = confirm_and_install(
            &confirmation,
            &installer,
            "Do you want to install cargo-watch ?",
            None,
            "Installing cargo-watch ...",
            &install_command(),
        );

        assert_eq!(outcome, ValidationOutcome::PromptFailed);
        assert!(installer.commands().is_empty());
    }

    #[test]
    fn test_which_outcomes_are_fatal() {
        assert!(!ValidationOutcome::AlreadyInstalled.is_fatal());
        assert!(!ValidationOutcome::Installed.is_fatal());
        assert!(ValidationOutcome::Declined.is_fatal());
        assert!(ValidationOutcome::PromptFailed.is_fatal());
        assert!(ValidationOutcome::Unsatisfiable.is_fatal());
    }
}
