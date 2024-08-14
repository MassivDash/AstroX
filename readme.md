# Astro X

Monolithic repo for developing full stack application, using rust and cargo tools as primary development environment.

Frontend is a standalone astro.build application that will create the frontend bundle served by rust actix server.

## Rust + Astro web development boilerplate.

To start developing with AstroX you will need rustc > 1.74 and node > 18.14. Clone the project and execute;

```
cargo run
```

That's all you need to get started, the interactive cli will guide you through installation process.

## Features

### CLI

Rust written command line interface starts, serves and tests the astro x project. Fast and efficient with only few dependencies will create a professional development environment for rust opinionated project.

#### Project Runner

Handles installation and system checks, it will check the astroX system prerequisites and either help you install or provide you with necessary information to start the project.

- automatic development port rotation for frontend and backend
- interactive mode, execute actions through cli gui
- git hooks integration
- build the packages
- serve the bundle (with auto restart)
- test the project
- execute the project with cmd line arguments

#### Git hooks

Pre defined git hooks for quality code writing

- commit msg via commitlint-rs
- pre-commit (test and lint staged files)
- pre-push (test all)

#### CLI arguments

```sh
Command list:
--help [print this help ]
--sync-git-hooks [copy git_hooks folder contents to .git/hooks]
--remove-git-hooks [remove hooks from .git/hooks folder]
--build [build production bundle for frontend and backend]
--serve [start the production server with the frontend build]
--test [run the tests]
--create-toml [create a new Astrox.toml file]
--interactive [start the interactive mode]
--system-checks [run the system checks]


Cli arguments:
--host="127.0.0.1" [ip address]
--port=8080 [actix port number]
--env=prod / dev [environment]
--astro-port=4321 [astro development port number]
--prod-astro-build=true / false [Build astro during cli prod start]
```

### Actix backend

https://actix.rs/docs/getting-started/

Rust based server from Actix framework.

- serve static astro x files
- 3rd api call example
- logging
- graphql [coming soon]
- ssr [coming soon]

### Astro

https://astro.build/

Astro is a frontend framework that focuses on mainly on delivering html first, the fastest and most versatile of the frameworks allows to incorporate any of the major UI frameworks such as React, Svelte, Vue, Solid.js and others ...

The boilerplate provides and example of the Astro 4.0 transition capabilities.

### Demo

https://astrox.spaceout.pl
