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
    hooks_are_installed_in(".git/hooks")
}

/// A hooks folder counts as installed when it exists and is not empty.
pub fn hooks_are_installed_in(hooks_dir: &str) -> bool {
    // Get the list of hooks from the hooks folder
    let hooks = fs::read_dir(hooks_dir);
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

    fn read_the_hooks() -> Vec<String> {
        let hooks = fs::read_dir(".git/hooks").unwrap();
        let mut hooks_list = Vec::new();
        for hook in hooks {
            let hook = hook.unwrap();
            let hook_name = hook.file_name();
            let hook_name = hook_name.to_str().unwrap();
            hooks_list.push(hook_name.to_string());
        }
        println!("{:?}", hooks_list);
        hooks_list
    }

    #[test]
    fn test_copy_git_hooks() {
        let check = check_if_git_hooks_are_installed();
        assert!(check == true);

        copy_git_hooks();

        // Assert
        // Verify that the hooks are copied to .git/hooks
        assert!(read_the_hooks()
            .iter()
            .any(|hook| hook == "pre-commit" || hook == "pre-push" || hook == "commit-msg"));
    }

    #[test]
    fn test_remove_git_hooks() {
        // Act
        remove_git_hooks();
        let hooks = read_the_hooks();
        // Assert
        // Verify that the hooks are removed from .git/hooks
        assert!(hooks.iter().any(|hook| hook == "pre-commit") == false);

        copy_git_hooks(); //return the githooks back after tests
    }

    #[test]
    fn test_hooks_are_not_installed_when_the_folder_is_missing() {
        assert_eq!(hooks_are_installed_in("no-such-folder/hooks"), false);
    }

    #[test]
    fn test_hooks_are_not_installed_when_the_folder_is_empty() {
        let empty_dir = std::env::temp_dir().join("astrox-test-empty-hooks");
        let empty_dir = empty_dir.to_str().unwrap();

        // Start from a clean, empty folder
        let _ = fs::remove_dir_all(empty_dir);
        fs::create_dir_all(empty_dir).unwrap();

        assert_eq!(hooks_are_installed_in(empty_dir), false);

        // A folder with a file in it counts as installed
        fs::write(format!("{}/pre-commit", empty_dir), "#!/bin/sh\n").unwrap();
        assert_eq!(hooks_are_installed_in(empty_dir), true);

        fs::remove_dir_all(empty_dir).unwrap();
    }
}
