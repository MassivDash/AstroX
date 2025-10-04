#[cfg(test)]
mod tests {
    use crate::cli::config::get_config::get_config;

    #[test]
    fn test_get_config_with_default_values() {
        let args: Vec<String> = vec![];
        let config = get_config(&args);

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, Some(8080));
        assert_eq!(config.env, "dev");
        assert_eq!(config.astro_port, Some(5431)); //overide from projects toml
        assert_eq!(config.prod_astro_build, true); //overide from projects toml
        assert_eq!(
            config.public_keys.public_api_url,
            "http://localhost:8080/api"
        );
        assert_eq!(config.cookie_domain, None); // Default is None for dev
    }

    #[test]
    fn test_get_config_with_custom_values() {
        let args: Vec<String> = vec![
            "--host=example.com".to_string(),
            "--port=8000".to_string(),
            "--env=prod".to_string(),
            "--astro-port=5431".to_string(),
            "--prod-astro-build=false".to_string(),
            "--public-api-url=https://api.example.com".to_string(),
            "--cookie-domain=.example.com".to_string(),
        ];
        let config = get_config(&args);

        assert_eq!(config.host, "example.com");
        assert_eq!(config.port, Some(8000));
        assert_eq!(config.env, "prod");
        assert_eq!(config.astro_port, Some(5431));
        assert_eq!(config.prod_astro_build, false);
        assert_eq!(config.public_keys.public_api_url, "https://api.example.com");
        assert_eq!(config.cookie_domain, Some(".example.com".to_string()));
    }

    #[test]
    fn test_get_config_with_invalid_args() {
        let args: Vec<String> = vec!["--invalid-arg".to_string()];
        let config = get_config(&args);

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, Some(8080));
        assert_eq!(config.env, "dev");
        assert_eq!(config.astro_port, Some(5431));
        assert_eq!(config.prod_astro_build, true);
        assert_eq!(
            config.public_keys.public_api_url,
            "http://localhost:8080/api"
        );
        assert_eq!(config.cookie_domain, None); // Default is None
    }

    #[test]
    fn test_get_config_with_cookie_domain() {
        let args: Vec<String> = vec!["--cookie-domain=.mydomain.com".to_string()];
        let config = get_config(&args);

        assert_eq!(config.cookie_domain, Some(".mydomain.com".to_string()));
    }
}
