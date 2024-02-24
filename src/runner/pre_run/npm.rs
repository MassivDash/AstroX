/// The name of the npm executable.
/// On windows it is npm.cmd
/// On linux and mac it is npm

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &str = "npm";
