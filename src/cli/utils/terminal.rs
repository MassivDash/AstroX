use crossterm::execute;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use std::io::stdout;

// Read the Cargo.toml file and get the version
pub fn get_version() -> String {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let version = cargo_toml
        .split('\n')
        .find(|line| line.contains("version"))
        .unwrap()
        .split('=')
        .collect::<Vec<&str>>()[1]
        .trim()
        .replace('\"', "");

    version
}

pub fn do_server_log(string: &str) {
    spacer();
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkRed),
        Print("|   Actix   |"),
        ResetColor,
        Print(" "),
        Print(string),
    )
    .unwrap();
    spacer();
}

pub fn do_front_log(string: &str) {
    spacer();
    execute!(
        stdout(),
        SetForegroundColor(Color::White),
        SetBackgroundColor(Color::DarkMagenta),
        Print("|   Astro   |"),
        ResetColor,
        Print(" "),
        Print(string),
    )
    .unwrap();
    spacer();
}

pub fn do_splash() {
    spacer();
    execute!(
        stdout(),
        SetForegroundColor(Color::Magenta),
        Print(
            "
  █████  ███████ ████████ ██████   ██████  ██   ██ 
  ██   ██ ██         ██    ██   ██ ██    ██  ██ ██  
  ███████ ███████    ██    ██████  ██    ██   ███   
  ██   ██      ██    ██    ██   ██ ██    ██  ██ ██  
  ██   ██ ███████    ██    ██   ██  ██████  ██   ██
"
        ),
        ResetColor
    )
    .unwrap();
    spacer();
    execute!(
        stdout(),
        Print(format!(
            "{} astro_x: version {} author: @spaceout.pl",
            ResetColor,
            get_version()
        )),
        ResetColor
    )
    .unwrap();
    spacer();
    hr();
    spacer();
}

pub fn hr() {
    execute!(
        stdout(),
        SetForegroundColor(Color::Magenta),
        Print(
            "=============================================================================================================================================="
        ),
        ResetColor
    ).unwrap();
}

pub fn spacer() {
    execute!(stdout(), ResetColor, Print("\n\n"), ResetColor).unwrap();
}

pub fn step(string: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Magenta),
        Print(format!("🏁 Action: {}\n", string)),
        ResetColor
    )
    .unwrap();
}

pub fn success(string: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print(format!("✅ Success: {}\n", string)),
        ResetColor
    )
    .unwrap();
}

pub fn warning(string: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print(format!("☢️ Warning: {}\n", string)),
        ResetColor
    )
    .unwrap();
}

pub fn dev_info(host: &String, port: &u16) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("| Local development backend server running at:\n"),
        Print(format!("| http://{}:{}\n", host, port)),
        Print("|\n"),
        ResetColor
    )
    .unwrap();
}

pub fn error(string: &str) {
    execute!(
        stdout(),
        SetForegroundColor(Color::Red),
        Print(format!("❗ Error: {}\n", string)),
        ResetColor
    )
    .unwrap();
}

pub fn help() {
    execute!(
        stdout(),
        SetForegroundColor(Color::Magenta),
        Print(format!("v{} --- Astrox CLI\n", get_version())),
        ResetColor
    )
    .unwrap();
    spacer();
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        Print("Command list:\n"),
        Print("--help [print this help ]\n"),
        Print("--sync-git-hooks [copy git_hooks folder contents to .git/hooks]\n"),
        Print("--remove-git-hooks [remove hooks from .git/hooks folder]\n"),
        Print("--build [build production bundle for frontend and backend]\n"),
        Print("--serve [start the production server with the frontend build]\n"),
        Print("--test [run the tests]\n"),
        Print("--create-toml [create a new Astrox.toml file]\n"),
        Print("--interactive [start the interactive mode]\n"),
        Print("--system-checks [run the system checks]\n"),
        ResetColor
    )
    .unwrap();
    spacer();
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        Print("Cli arguments:\n"),
        Print("--host=127.0.0.1 [ip address]\n"),
        Print("--port=8080 [actix port number]\n"),
        Print("--env=prod / dev [environment]\n"),
        Print("--astro-port=4321 [astro development port number]\n"),
        Print("--prod-astro-build=true / false [Build astro during cli prod start]\n"),
        ResetColor
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version() {
        // Test when Cargo.toml contains version
        let version = get_version();
        assert_eq!(version, "0.1.1");
    }

    #[test]
    fn test_do_splash() {
        // Test the output of do_splash function
        do_splash();
    }

    #[test]
    fn test_hr() {
        // Test the output of hr function
        hr();
    }

    #[test]
    fn test_spacer() {
        // Test the output of spacer function
        spacer();
    }

    #[test]
    fn test_step() {
        // Test the output of step function
        step("Test Step");
    }

    #[test]
    fn test_success() {
        // Test the output of success function
        success("Test Success");
    }

    #[test]
    fn test_warning() {
        // Test the output of warning function
        warning("Test Warning");
    }

    #[test]
    fn test_dev_info() {
        // Test the output of dev_info function
        let host = String::from("localhost");
        let port = 8080;
        dev_info(&host, &port);
    }

    #[test]
    fn test_error() {
        // Test the output of error function
        error("Test Error");
    }

    #[test]
    fn test_help() {
        // Test the output of help function
        help();
    }
}
