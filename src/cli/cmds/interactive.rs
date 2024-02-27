use super::execute_cmd::execute_cmd;
use inquire::Select;

pub fn start_interactive() {
    let options = vec!["Sync git hooks", "Create toml file", "System check", "Run"];

    let select = Select::new("Select a command to run", options);
    let selected = select.prompt().unwrap_or("Run");

    match selected {
        "Sync git hooks" => {
            execute_cmd(&vec!["--sync-git-hooks".to_string()]);
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
        _ => {
            execute_cmd(&vec!["--run".to_string()]);
        }
    }
}
