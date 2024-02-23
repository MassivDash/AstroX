use std::fs;

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
    }
}
