use std::cell::RefCell;

use crate::cli::cmds::interactive::{CommandExecutor, UserInput};

struct MockUserInput {
    responses: RefCell<Vec<String>>,
}

impl MockUserInput {
    fn new(responses: Vec<String>) -> Self {
        Self {
            responses: RefCell::new(responses),
        }
    }
}

impl UserInput for MockUserInput {
    fn select(&self, _prompt: &str, _options: Vec<&str>) -> String {
        self.responses.borrow_mut().remove(0)
    }
}

struct MockCommandExecutor {
    executed_commands: RefCell<Vec<String>>,
}

impl MockCommandExecutor {
    fn new() -> Self {
        Self {
            executed_commands: RefCell::new(vec![]),
        }
    }
}

impl CommandExecutor for MockCommandExecutor {
    fn execute_command(&self, command: &str) {
        self.executed_commands
            .borrow_mut()
            .push(command.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::cmds::interactive::start_interactive;

    use super::*;

    #[test]
    fn test_start_interactive_run() {
        let mock_input = MockUserInput::new(vec!["Run".to_string()]);
        let mock_executor = MockCommandExecutor::new();
        start_interactive(&mock_input, &mock_executor);
        // Add assertions to verify the behavior
        let executed_commands = mock_executor.executed_commands.borrow();
        assert_eq!(executed_commands.len(), 1);
        assert_eq!(executed_commands[0], "--run");
    }

    #[test]
    fn test_start_interactive_build() {
        let mock_input = MockUserInput::new(vec!["Build".to_string()]);
        let mock_executor = MockCommandExecutor::new();
        start_interactive(&mock_input, &mock_executor);

        let executed_commands = mock_executor.executed_commands.borrow();
        assert_eq!(executed_commands.len(), 1);
        assert_eq!(executed_commands[0], "--build");
    }

    #[test]
    fn test_start_interactive_serve() {
        let mock_input = MockUserInput::new(vec!["Serve".to_string()]);
        let mock_executor = MockCommandExecutor::new();
        start_interactive(&mock_input, &mock_executor);

        let executed_commands = mock_executor.executed_commands.borrow();
        assert_eq!(executed_commands.len(), 1);
        assert_eq!(executed_commands[0], "--serve");
    }

    #[test]
    fn test_start_interactive_test() {
        let mock_input = MockUserInput::new(vec!["Test".to_string()]);
        let mock_executor = MockCommandExecutor::new();
        start_interactive(&mock_input, &mock_executor);

        let executed_commands = mock_executor.executed_commands.borrow();
        assert_eq!(executed_commands.len(), 1);
        assert_eq!(executed_commands[0], "--test");
    }

    #[test]
    fn test_start_interactive_create_toml_file() {
        let mock_input = MockUserInput::new(vec!["Create toml file".to_string()]);
        let mock_executor = MockCommandExecutor::new();
        start_interactive(&mock_input, &mock_executor);

        let executed_commands = mock_executor.executed_commands.borrow();
        assert_eq!(executed_commands.len(), 1);
        assert_eq!(executed_commands[0], "--create-toml");
    }

    #[test]
    fn test_start_interactive_sync_git_hooks() {
        let mock_input = MockUserInput::new(vec!["Sync git hooks".to_string()]);
        let mock_executor = MockCommandExecutor::new();
        start_interactive(&mock_input, &mock_executor);

        let executed_commands = mock_executor.executed_commands.borrow();
        assert_eq!(executed_commands.len(), 1);
        assert_eq!(executed_commands[0], "--sync-git-hooks");
    }

    #[test]
    fn test_start_interactive_remove_git_hooks() {
        let mock_input = MockUserInput::new(vec!["Remove git hooks".to_string()]);
        let mock_executor = MockCommandExecutor::new();
        start_interactive(&mock_input, &mock_executor);

        let executed_commands = mock_executor.executed_commands.borrow();
        assert_eq!(executed_commands.len(), 1);
        assert_eq!(executed_commands[0], "--remove-git-hooks");
    }

    #[test]
    fn test_start_interactive_system_check() {
        let mock_input = MockUserInput::new(vec!["System check".to_string()]);
        let mock_executor = MockCommandExecutor::new();
        start_interactive(&mock_input, &mock_executor);

        let executed_commands = mock_executor.executed_commands.borrow();
        assert_eq!(executed_commands.len(), 1);
        assert_eq!(executed_commands[0], "--system-check");
    }
}
