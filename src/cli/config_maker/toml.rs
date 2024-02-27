use super::config::Config;
use crate::cli::utils::terminal::{error, spacer, step, success, warning};

pub fn read_toml(filename: String) -> Result<Config, ()> {
    let toml_str = std::fs::read_to_string(filename);

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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_toml_success() {
        // Arrange
        let expected_config = Config {
            host: "127.0.0.1".to_string(),
            port: Some(8080),
            env: "dev".to_string(),
            astro_port: Some(5431),
            prod_astro_build: true,
        };

        let file_name: String = "Astrox-test.toml".to_string();

        let toml_str = toml::to_string(&expected_config).unwrap();
        std::fs::write(file_name, toml_str).unwrap();

        // Act
        let result = read_toml(file_name);

        // Assert
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.host == expected_config.host);
        assert!(config.port == expected_config.port);
        assert!(config.env == expected_config.env);
        assert!(config.astro_port == expected_config.astro_port);
        assert!(config.prod_astro_build == expected_config.prod_astro_build);

        // delete file after test completion
        std::fs::remove_file(file_name);
    }

    #[test]
    fn test_read_toml_file_not_found() {
        let file_name: String = "Astrox-not.toml".to_string();
        let result = read_toml(file_name);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_read_toml_parse_error() {
        let file_name: String = "Astrox-test.toml".to_string();
        std::fs::write("Astrox-test.toml", "invalid toml").unwrap();

        // Act
        let result = read_toml(file_name);

        // Assert
        assert!(result.is_err());

        // delete file after test completion
        std::fs::remove_file(file_name);
    }
}
