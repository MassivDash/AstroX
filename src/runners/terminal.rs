
use termion::color;

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
        "{} astro_x: version 0.1.0 author: @spaceout.pl",
        color::Fg(color::Reset)
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
    println!("{}", color::Fg(color::LightGreen));
    println!("{}", string);
    println!("{}", color::Fg(color::Reset));
}
