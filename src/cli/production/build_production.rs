use crate::cli::config::create_dotenv::create_dotenv_frontend;
use crate::cli::config::get_config::{get_prod_config, Config};
use crate::cli::pre_run::npm::checks::NPM;
use crate::cli::utils::terminal::step;
use std::process::Command;

/// Runs a build command and reports whether it succeeded.
pub trait BuildRunner {
    fn run(&self, program: &str, args: &[&str], dir: &str, failure_message: &str) -> bool;
}

pub struct RealBuildRunner;

impl BuildRunner for RealBuildRunner {
    fn run(&self, program: &str, args: &[&str], dir: &str, failure_message: &str) -> bool {
        Command::new(program)
            .args(args)
            .current_dir(dir)
            .spawn()
            .expect(failure_message)
            .wait()
            .expect(failure_message)
            .success()
    }
}

/// Build the production bundles.
/// The frontend is only bundled when `prod_astro_build` is on.
/// `dotenv_path` is the frontend .env file the public keys are written to.
pub fn build_production_with(config: Config, runner: &dyn BuildRunner, dotenv_path: &str) {
    if config.prod_astro_build {
        let prod_build_url = config.public_keys.public_api_url;

        create_dotenv_frontend(&prod_build_url, dotenv_path);

        step("Building the frontend package");

        match runner.run(
            NPM,
            &["run", "build"],
            "./src/frontend",
            "Failed to bundle the frontend",
        ) {
            true => step("Frontend bundled successfully"),
            false => panic!("Failed to bundle the frontend"),
        }
    }

    step("Building cargo backend production server");

    match runner.run(
        "cargo",
        &["build", "--release"],
        "./src/backend",
        "Failed to start backend production server",
    ) {
        true => step("Backend built successfully"),
        false => panic!("Failed to build the backend"),
    }
}

pub fn build_production(config: Config) {
    build_production_with(config, &RealBuildRunner, "./src/frontend/.env")
}

pub fn execute_build() {
    build_production(get_prod_config())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::config::get_config::PublicKeys;
    use std::cell::RefCell;

    struct FakeBuildRunner {
        succeeds: bool,
        calls: RefCell<Vec<String>>,
    }

    impl FakeBuildRunner {
        fn new(succeeds: bool) -> Self {
            Self {
                succeeds,
                calls: RefCell::new(vec![]),
            }
        }
    }

    impl BuildRunner for FakeBuildRunner {
        fn run(&self, program: &str, args: &[&str], dir: &str, _failure_message: &str) -> bool {
            self.calls
                .borrow_mut()
                .push(format!("{} {} @ {}", program, args.join(" "), dir));
            self.succeeds
        }
    }

    fn config(prod_astro_build: bool) -> Config {
        Config {
            host: "localhost".to_string(),
            port: Some(8080),
            env: "prod".to_string(),
            astro_port: Some(5431),
            cors_url: "http://localhost:5431".to_string(),
            prod_astro_build,
            cookie_domain: None,
            public_keys: PublicKeys {
                public_api_url: "https://api.example.com".to_string(),
            },
        }
    }

    fn temp_dotenv(name: &str) -> String {
        std::env::temp_dir()
            .join(name)
            .to_str()
            .unwrap()
            .to_string()
    }

    #[test]
    fn test_build_production_bundles_the_frontend_and_the_backend() {
        let runner = FakeBuildRunner::new(true);
        let dotenv_path = &temp_dotenv("astrox-test-build-production.env");
        let _ = std::fs::remove_file(dotenv_path);

        build_production_with(config(true), &runner, dotenv_path);

        assert_eq!(
            *runner.calls.borrow(),
            vec![
                format!("{} run build @ ./src/frontend", NPM),
                "cargo build --release @ ./src/backend".to_string(),
            ]
        );

        let dotenv = std::fs::read_to_string(dotenv_path).unwrap();
        assert!(dotenv.contains("PUBLIC_API_URL=https://api.example.com"));

        std::fs::remove_file(dotenv_path).unwrap();
    }

    #[test]
    fn test_build_production_skips_the_frontend_when_switched_off() {
        let runner = FakeBuildRunner::new(true);
        let dotenv_path = &temp_dotenv("astrox-test-build-production-skipped.env");
        let _ = std::fs::remove_file(dotenv_path);

        build_production_with(config(false), &runner, dotenv_path);

        assert_eq!(
            *runner.calls.borrow(),
            vec!["cargo build --release @ ./src/backend".to_string()]
        );
        assert!(!std::path::Path::new(dotenv_path).exists());
    }

    #[test]
    #[should_panic(expected = "Failed to bundle the frontend")]
    fn test_build_production_panics_when_the_frontend_bundle_fails() {
        let runner = FakeBuildRunner::new(false);
        let dotenv_path = &temp_dotenv("astrox-test-build-production-failing.env");

        build_production_with(config(true), &runner, dotenv_path);
    }

    #[cfg(unix)]
    #[test]
    fn test_real_build_runner_reports_the_exit_status() {
        assert!(RealBuildRunner.run("true", &[], ".", "true should never fail"));
        assert!(!RealBuildRunner.run("false", &[], ".", "false should never fail"));
    }

    #[test]
    #[should_panic(expected = "Failed to build the backend")]
    fn test_build_production_panics_when_the_backend_build_fails() {
        let runner = FakeBuildRunner::new(false);
        let dotenv_path = &temp_dotenv("astrox-test-build-production-unused.env");

        build_production_with(config(false), &runner, dotenv_path);
    }
}
