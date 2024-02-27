use super::config::Config;
use crate::cli::utils::terminal::{error, spacer, step, success, warning};

pub fn read_toml() -> Result<Config, ()> {
    let toml_str = std::fs::read_to_string("Astrox.toml");

    match toml_str {
        Ok(toml_str) => {
            success("Astrox.toml found");

            let config = toml::from_str(&toml_str);
            match config {
                Ok(config) => {
                    step("loaded Astrox.toml");
                    Ok(config)
                }
                Err(e) => {
                    error("Failed to parse Astrox.toml");
                    error(e.to_string().as_str());
                    spacer();
                    Err(())
                }
            }
        }
        Err(_) => {
            warning("Astrox.toml not found");
            spacer();
            Err(())
        }
    }
}
