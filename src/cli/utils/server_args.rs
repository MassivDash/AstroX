/// The arguments handed over to the actix backend binary.
///
/// `env` and `cors_url` are optional because the development server does not
/// pass them, while the production server does.
pub struct BackendArgs<'a> {
    pub host: &'a str,
    pub port: u16,
    pub env: Option<&'a str>,
    pub cors_url: Option<&'a str>,
    pub cookie_domain: Option<&'a str>,
}

impl BackendArgs<'_> {
    /// Build the `--flag=value` arguments passed to the backend binary.
    pub fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            format!("--host={}", self.host),
            format!("--port={}", self.port),
        ];

        if let Some(env) = self.env {
            args.push(format!("--env={}", env));
        }
        if let Some(cors_url) = self.cors_url {
            args.push(format!("--cors_url={}", cors_url));
        }
        if let Some(cookie_domain) = self.cookie_domain {
            args.push(format!("--cookie_domain={}", cookie_domain));
        }

        args
    }

    /// Build the command string handed to `cargo watch -x`.
    pub fn to_watch_command(&self) -> String {
        format!("run -- {}", self.to_args().join(" "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dev_args() -> BackendArgs<'static> {
        BackendArgs {
            host: "localhost",
            port: 8080,
            env: None,
            cors_url: None,
            cookie_domain: None,
        }
    }

    #[test]
    fn test_minimal_development_args() {
        assert_eq!(
            dev_args().to_args(),
            vec!["--host=localhost", "--port=8080",]
        );
    }

    #[test]
    fn test_watch_command_is_the_joined_args() {
        assert_eq!(
            dev_args().to_watch_command(),
            "run -- --host=localhost --port=8080"
        );
    }

    #[test]
    fn test_watch_command_with_every_optional_flag() {
        let args = BackendArgs {
            host: "0.0.0.0",
            port: 9000,
            env: None,
            cors_url: None,
            cookie_domain: Some(".spaceout.pl"),
        };

        assert_eq!(
            args.to_watch_command(),
            "run -- --host=0.0.0.0 --port=9000 --cookie_domain=.spaceout.pl"
        );
    }

    #[test]
    fn test_production_args_order() {
        let args = BackendArgs {
            host: "0.0.0.0",
            port: 8080,
            env: Some("prod"),
            cors_url: Some("http://192.168.0.56"),
            cookie_domain: None,
        };

        assert_eq!(
            args.to_args(),
            vec![
                "--host=0.0.0.0",
                "--port=8080",
                "--env=prod",
                "--cors_url=http://192.168.0.56",
            ]
        );
    }

    #[test]
    fn test_production_args_with_all_optional_flags() {
        let args = BackendArgs {
            host: "0.0.0.0",
            port: 8080,
            env: Some("prod"),
            cors_url: Some("http://192.168.0.56"),
            cookie_domain: Some(".spaceout.pl"),
        };

        assert_eq!(
            args.to_args(),
            vec![
                "--host=0.0.0.0",
                "--port=8080",
                "--env=prod",
                "--cors_url=http://192.168.0.56",
                "--cookie_domain=.spaceout.pl",
            ]
        );
    }
}
