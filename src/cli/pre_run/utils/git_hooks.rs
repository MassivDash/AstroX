use std::fs;

/// This function will copy the git hooks from the git_hooks folder to .git/hooks
/// This will allow the user to use the pre-commit, pre-push and commit-msg hooks
/// The hooks will help the user to maintain the code quality
/// Alter the hooks in the git_hooks folder to fit your needs

pub fn copy_git_hooks() {
    // Get the list of hooks from the git_hooks folder
    let hooks = fs::read_dir("git_hooks").unwrap();

    // For each hook, copy the file to .git/hooks

    for hook in hooks {
        let hook = hook.unwrap();
        let hook_name = hook.file_name();
        let hook_name = hook_name.to_str().unwrap();
        let hook_path = hook.path();
        let hook_path = hook_path.to_str().unwrap();

        let git_hook_path = format!(".git/hooks/{}", hook_name);
        let git_hook_path = git_hook_path.as_str();

        // Copy the hook to .git/hooks
        match fs::copy(hook_path, git_hook_path) {
            Ok(_) => {
                println!("{} copied to {}", hook_name, git_hook_path);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        // Make the hook executable for all systems (linux, mac, windows)

        #[cfg(not(windows))]
        {
            let output = std::process::Command::new("chmod")
                .arg("+x")
                .arg(git_hook_path)
                .output()
                .expect("Failed to execute command");
            let output_str = String::from_utf8_lossy(&output.stdout);
            println!("{}", output_str);
        }

        #[cfg(windows)]
        {
            // No need to make the file executable on windows
        }
    }
}

// Try and find match between git_hooks and .git/hooks
pub fn check_if_git_hooks_are_installed() -> bool {
    // Get the list of hooks from the .git/hooks folder
    let hooks = fs::read_dir(".git/hooks");
    if hooks.is_err() {
        return false;
    }
    let hooks = hooks.unwrap();
    // Check if the folder is empty
    if hooks.count() == 0 {
        return false;
    }
    true
}

pub fn remove_git_hooks() {
    // Get the list of hooks from the git_hooks folder
    let hooks = fs::read_dir("git_hooks").unwrap();

    // For each hook, copy the file to .git/hooks

    for hook in hooks {
        let hook = hook.unwrap();
        let hook_name = hook.file_name();
        let hook_name = hook_name.to_str().unwrap();
        let git_hook_path = format!(".git/hooks/{}", hook_name);
        let git_hook_path = git_hook_path.as_str();

        // Remove the hook from .git/hooks
        match fs::remove_file(git_hook_path) {
            Ok(_) => {
                println!("{} removed from {}", hook_name, git_hook_path);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_git_hooks() {
        // Arrange

        // Act
        copy_git_hooks();

        // Assert
        // Verify that the hooks are copied to .git/hooks
        assert!(check_if_git_hooks_are_installed());
    }

    #[test]
    fn test_remove_git_hooks() {
        // Arrange
        copy_git_hooks();

        // Act
        remove_git_hooks();

        // Assert
        // Verify that the hooks are removed from .git/hooks
        assert!(!check_if_git_hooks_are_installed());
    }
}
