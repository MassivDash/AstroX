#[cfg(test)]
mod tests {
    use crate::cli::config::get_config::{get_config, get_config_from, get_prod_config};

    fn write_fixture_toml(name: &str) -> String {
        let path = std::env::temp_dir()
            .join(name)
            .to_str()
            .unwrap()
            .to_string();

        std::fs::write(
            &path,
            r#"
host = "10.0.0.1"
port = 9090
env = "dev"
astro_port = 5431
prod_astro_build = true
cors_url = "http://10.0.0.1"

[public_keys]
public_api_url = "http://10.0.0.1:9090/api"
"#,
        )
        .unwrap();

        path
    }

    #[test]
    fn test_get_config_with_default_values() {
        let args: Vec<String> = vec![];
        let config = get_config_from("Astrox-does-not-exist.toml", &args);

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, Some(8080));
        assert_eq!(config.env, "dev");
        assert_eq!(config.astro_port, Some(5432));
        assert_eq!(config.prod_astro_build, false);
        assert_eq!(config.cors_url, "http://localhost:5432");
        assert_eq!(
            config.public_keys.public_api_url,
            "http://localhost:8080/api"
        );
        assert_eq!(config.cookie_domain, None);
    }

    #[test]
    fn test_get_config_takes_the_toml_over_the_defaults() {
        let toml_path = write_fixture_toml("astrox-get-config-toml.toml");
        let args: Vec<String> = vec![];

        let config = get_config_from(&toml_path, &args);

        assert_eq!(config.host, "10.0.0.1");
        assert_eq!(config.port, Some(9090));
        assert_eq!(config.astro_port, Some(5431));
        assert_eq!(config.prod_astro_build, true);
        assert_eq!(
            config.public_keys.public_api_url,
            "http://10.0.0.1:9090/api"
        );

        std::fs::remove_file(&toml_path).unwrap();
    }

    #[test]
    fn test_get_config_with_custom_values() {
        let toml_path = write_fixture_toml("astrox-get-config-args.toml");
        let args: Vec<String> = vec![
            "--host=example.com".to_string(),
            "--port=8000".to_string(),
            "--env=prod".to_string(),
            "--astro-port=5431".to_string(),
            "--prod-astro-build=false".to_string(),
            "--public-api-url=https://api.example.com".to_string(),
            "--cookie-domain=.example.com".to_string(),
        ];

        let config = get_config_from(&toml_path, &args);

        assert_eq!(config.host, "example.com");
        assert_eq!(config.port, Some(8000));
        assert_eq!(config.env, "prod");
        assert_eq!(config.astro_port, Some(5431));
        assert_eq!(config.prod_astro_build, false);
        assert_eq!(config.public_keys.public_api_url, "https://api.example.com");
        assert_eq!(config.cookie_domain, Some(".example.com".to_string()));

        std::fs::remove_file(&toml_path).unwrap();
    }

    #[test]
    fn test_get_config_with_invalid_args() {
        let args: Vec<String> = vec!["--invalid-arg".to_string()];
        let config = get_config_from("Astrox-does-not-exist.toml", &args);

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, Some(8080));
        assert_eq!(config.env, "dev");
        assert_eq!(config.astro_port, Some(5432));
        assert_eq!(config.prod_astro_build, false);
        assert_eq!(
            config.public_keys.public_api_url,
            "http://localhost:8080/api"
        );
        assert_eq!(config.cookie_domain, None);
    }

    #[test]
    fn test_get_config_with_cookie_domain() {
        let args: Vec<String> = vec!["--cookie-domain=.mydomain.com".to_string()];
        let config = get_config(&args);

        assert_eq!(config.cookie_domain, Some(".mydomain.com".to_string()));
    }

    #[test]
    fn test_get_config_reads_the_projects_toml() {
        let args: Vec<String> = vec!["--host=from-args.example.com".to_string()];
        let config = get_config(&args);

        assert_eq!(config.host, "from-args.example.com");
    }

    #[test]
    fn test_get_prod_config_forces_the_prod_environment() {
        let config = get_prod_config();

        assert_eq!(config.env, "prod");
        assert!(config.port.is_some());
        assert!(!config.host.is_empty());
    }
}
