use super::execute_cmd::execute_cmd;
use inquire::Select;

pub trait UserInput {
    fn select(&self, prompt: &str, options: Vec<&str>) -> String;
}

pub struct InquireUserInput;

impl UserInput for InquireUserInput {
    fn select(&self, prompt: &str, options: Vec<&str>) -> String {
        let select = Select::new(prompt, options);
        select.prompt().unwrap_or("Run").to_string()
    }
}
pub trait CommandExecutor {
    fn execute_command(&self, command: &str);
}

pub struct RealCommandExecutor;

impl CommandExecutor for RealCommandExecutor {
    fn execute_command(&self, command: &str) {
        execute_cmd(&vec![command.to_string()]);
    }
}

pub fn start_interactive(user_input: &dyn UserInput, command_executor: &dyn CommandExecutor) {
    let options = vec![
        "Run",
        "Build",
        "Serve",
        "Test",
        "Coverage",
        "Create toml file",
        "Sync git hooks",
        "Remove git hooks",
        "System check",
    ];

    let selected = user_input.select("Select a command to run", options);

    let cmd = match selected.as_str() {
        "Sync git hooks" => "--sync-git-hooks",
        "Remove git hooks" => "--remove-git-hooks",
        "Create toml file" => "--create-toml",
        "System check" => "--system-check",
        "Run" => "--run",
        "Build" => "--build",
        "Serve" => "--serve",
        "Test" => "--test",
        "Coverage" => "--coverage",
        _ => "--run",
    };

    command_executor.execute_command(cmd);
}
