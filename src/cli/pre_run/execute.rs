use super::{system_checks::run_system_checks, utils::git_hooks};

pub fn execute(env: &str) {
    if !git_hooks::check_if_git_hooks_are_installed() {
        git_hooks::copy_git_hooks();
    }

    run_system_checks(env);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_with_installed_git_hooks() {
        // Arrange
        let env = "dev";
        // Act
        execute(env);
        // Assert
        // Add your assertions here to verify the expected behavior
    }
}
