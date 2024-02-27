use termion::color::{Fg, LightBlue, LightGreen, LightMagenta, LightRed, LightYellow, Reset};

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

pub fn do_splash() {
    spacer();
    println!(
        "{}
  █████  ███████ ████████ ██████   ██████  ██   ██ 
  ██   ██ ██         ██    ██   ██ ██    ██  ██ ██  
  ███████ ███████    ██    ██████  ██    ██   ███   
  ██   ██      ██    ██    ██   ██ ██    ██  ██ ██  
  ██   ██ ███████    ██    ██   ██  ██████  ██   ██
",
        Fg(LightMagenta)
    );
    spacer();
    println!(
        "{} astro_x: version {} author: @spaceout.pl",
        Fg(Reset),
        get_version()
    );
    hr();
}

pub fn hr() {
    println!("{}", Fg(LightMagenta));
    println!(
        "=============================================================================================================================================="
    );
    println!("{}", Fg(Reset));
}

pub fn spacer() {
    println!("{}", Fg(Reset));
    println!("{}", Fg(Reset));
}

pub fn step(string: &str) {
    println!("{}🏁 Action: {}", Fg(LightMagenta), string);
    println!("{}", Fg(Reset));
}

pub fn success(string: &str) {
    println!("{}✅ Success: {}", Fg(LightGreen), string);
    println!("{}", Fg(Reset));
}

pub fn warning(string: &str) {
    println!("{}", Fg(LightYellow));
    println!("☢️ Warning: {}", string);
    println!("{}", Fg(Reset));
}

// print
// | Local http://localhost:8080

pub fn dev_info(host: &String, port: &u16) {
    println!("{}|", Fg(LightGreen));
    println!(
        "{}| Local development backend server running at:",
        Fg(LightGreen)
    );
    println!("{}| http://{}:{}", Fg(LightGreen), host, port);
    println!("{}|", Fg(LightGreen));
    println!();
}

pub fn error(string: &str) {
    println!("{}❗ Error: {}", Fg(LightRed), string);
    println!("{}", Fg(Reset));
}

/// List of config arguments
/// Bind actix server to a host, used for development and production
/// --host=127.0.0.1
/// Bind actix server to a port, used for development and production
/// --port=8080
/// Set the environment
/// --env=prod / dev
/// Set the astro development port, in production actix server will serve the frontend build Files
/// --astro-port=4321
/// Switch on / off the production build of the frontend during the production server start
/// --prod-astro-build=true / false

/// List of commands
/// --help
/// --sync-git-hooks
/// --create-toml
/// --interactive
/// --system-checks

pub fn help() {
    println!("{}v{} --- Astrox CLI", Fg(LightMagenta), get_version());
    spacer();
    println!("{}Command list:", Fg(LightBlue));
    println!("{}--help [print this help ]", Fg(LightBlue));
    println!(
        "{}--sync-git-hooks [copy git_hooks folder contents to .git/hooks]",
        Fg(LightBlue)
    );
    println!(
        "{}--create-toml [create a new Astrox.toml file]",
        Fg(LightBlue)
    );
    println!(
        "{}--interactive [start the interactive mode]",
        Fg(LightBlue)
    );
    println!("{}--system-checks [run the system checks]", Fg(LightBlue));

    spacer();

    println!("{}Cli arguments:", Fg(LightBlue));
    println!("{}--host=127.0.0.1 [ip address]", Fg(LightBlue));
    println!("{}--port=8080 [actix port number]", Fg(LightBlue));
    println!("{}--env=prod / dev [environment]", Fg(LightBlue));
    println!(
        "{}--astro-port=4321 [astro development port number]",
        Fg(LightBlue)
    );

    println!(
        "{}--prod-astro-build=true / false [Build astro during cli prod start]",
        Fg(LightBlue)
    );
}
