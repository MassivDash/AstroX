use super::execute_cmd::execute_cmd;
use inquire::Select;

pub fn start_interactive() {
    let options = vec![
        "Run",
        "Build",
        "Serve",
        "Test",
        "Create toml file",
        "Sync git hooks",
        "Remove git hooks",
        "System check",
    ];

    let select = Select::new("Select a command to run", options);
    let selected = select.prompt().unwrap_or("Run");

    match selected {
        "Sync git hooks" => {
            execute_cmd(&vec!["--sync-git-hooks".to_string()]);
        }
        "Remove git hooks" => {
            execute_cmd(&vec!["--remove-git-hooks".to_string()]);
        }
        "Create toml file" => {
            execute_cmd(&vec!["--create-toml".to_string()]);
        }
        "System check" => {
            execute_cmd(&vec!["--system-check".to_string()]);
        }
        "Run" => {
            execute_cmd(&vec!["--run".to_string()]);
        }
        "Build" => {
            execute_cmd(&vec!["--build".to_string()]);
        }
        "Serve" => {
            execute_cmd(&vec!["--serve".to_string()]);
        }
        "Test" => {
            execute_cmd(&vec!["--test".to_string()]);
        }
        _ => {
            execute_cmd(&vec!["--run".to_string()]);
        }
    }
}
