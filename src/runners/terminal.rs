
use termion::color;

// Read the Cargo.toml file and get the version

pub fn get_version() -> String {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");
    let version = cargo_toml
        .split("\n")
        .find(|line| line.contains("version"))
        .unwrap()
        .split("=")
        .collect::<Vec<&str>>()[1]
        .trim()
        .replace("\"", "");

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
        color::Fg(color::Magenta)
    );
    spacer();
    println!(
        "{} astro_x: version {} author: @spaceout.pl",
        color::Fg(color::Reset), get_version()
    );
    hr();
}

pub fn hr() {
    println!("{}", color::Fg(color::LightMagenta));
    println!(
        "{}",
        "=============================================================================================================================================="
    );
    println!("{}", color::Fg(color::Reset));
}

pub fn spacer() {
    println!("{}", color::Fg(color::Reset));
    println!("{}", color::Fg(color::Reset));
}

pub fn step(string: &str) {
    println!("{}{}",color::Fg(color::LightGreen), string);
    println!("{}", color::Fg(color::Reset));
}

pub fn warning(string: &str) {
    println!("{}", color::Fg(color::LightYellow));
    println!("Warning: {}", string);
    println!("{}", color::Fg(color::Reset));
}

// print 
// | Local http://localhost:8080

pub fn dev_info(host: &str, port: &u16) {
    println!("{}Starting cargo backend development server", color::Fg(color::LightGreen));
    println!();
    println!("{}| Local http://{}:{}", color::Fg(color::Reset), host, port);
    println!();
}

pub fn error(string: &str) {
    println!("{}❗ Error: {}",color::Fg(color::LightRed), string);
    println!("{}", color::Fg(color::Reset));
}

