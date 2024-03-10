use std::path::Path;

/// This function will determine if the backend project was installed
/// Looking for src/backend/target folder
/// If the folder is not found, the function will return false and we can assume that this is the first cargo run activated by the user

pub fn is_first_run() -> bool {
    let path = Path::new("./src/backend/target/debug");
    !path.exists()
}
