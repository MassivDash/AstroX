<h1 align="center">AstroX (Actix + Astro.build)  </h1>

🦀 Rust orientated monolithic template for building modern web
        applications.
</h2> </div> </div>

<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.1.3-blue.svg?cacheSeconds=2592000" />
  <a href="https://github.com/MassivDash/astrox" target="_blank">
    <img alt="Documentation" src="https://img.shields.io/badge/documentation-yes-brightgreen.svg" />
  </a>
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
  <a href="https://twitter.com/SpaceoutPl" target="_blank">
    <img alt="Twitter: SpaceoutPl" src="https://img.shields.io/twitter/follow/SpaceoutPl.svg?style=social" />
  </a>
</p>

[![CodeQL](https://github.com/MassivDash/AstroX/actions/workflows/codeql-analysis.yml/badge.svg?branch=main)](https://github.com/MassivDash/AstroX/actions/workflows/codeql-analysis.yml)
[![Release](https://github.com/MassivDash/AstroX/actions/workflows/relase.yml/badge.svg?branch=main)](https://github.com/MassivDash/AstroX/actions/workflows/relase.yml)


**Platforms**

![windows](https://img.shields.io/badge/Platform-Windows-blue)
![linux](https://img.shields.io/badge/Platform-Linux-blue)
![macOs](https://img.shields.io/badge/Platform-MacOs-blue)

## Monolithic repo for developing full stack application, using rust and cargo tools as primary development environment.

Frontend is a standalone astro.build application that will create the frontend bundle served by rust actix server.

## Rust + Astro web development boilerplate.

To start developing with AstroX you will need rustc > 1.74 and node > 20.9.0 Clone the project and execute;

```
cargo run
```

That's all you need to get started, the interactive cli will guide you through installation process.


### Demo

https://astrox.spaceout.pl

## Features

### CLI

Rust written command line interface starts, serves and tests the astro x project. Fast and efficient with only few dependencies will create a professional development environment for rust opinionated project.

#### Cli Project Runner

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
--coverage [run cli and backend coverage]


Cli arguments:
--host="127.0.0.1" [ip address]
--port=8080 [actix port number]
--env=prod / dev [environment]
--astro-port=4321 [astro development port number]
--prod-astro-build=true / false [Build astro during cli prod start]
--set-public-api=https://custom.api/api [cli to astro env creation, used for static server url call building]
--cookie-domain=spaceout.pl [set domain for cookie enabled session]
```

### Actix backend

https://actix.rs/docs/getting-started/

Rust based server from Actix framework.

- serve static astro x files
- 3rd api call example
- logging
- ssr login page with session

### Astro

https://astro.build/

**Dev Helpers**

![eslint](https://img.shields.io/badge/ESLint-9.36.0-purple?logo=eslint)
![prettier](https://img.shields.io/badge/Prettier-3.6.2-blueviolet?logo=prettier)
![vitest](https://img.shields.io/badge/Vitest-3.2.4-yellowgreen?logo=vitest)
![testing-library](https://img.shields.io/badge/Testing_Library-6.9.1-red?logo=testinglibrary)
![typescript](https://img.shields.io/badge/TypeScript-5.9.3-3178c6?logo=typescript)
![svelte](https://img.shields.io/badge/Svelte-5.39.8-orange?logo=svelte)
![react](https://img.shields.io/badge/React-19.2.0-61dafb?logo=react)

Astro is a frontend framework that focuses on mainly on delivering html first, the fastest and most versatile of the frameworks allows to incorporate any of the major UI frameworks such as React, Svelte, Vue, Solid.js and others ...

The boilerplate provides and example of the Astro 5.0 transition capabilities.

## Project Structure

```bash
AstroX
├─ .github //workflows and ci/cd checks
├─ └─ workflows
├─ └─ ├─ codeql-analysis.yml
├─ └─ ├─ pr.yml
├─ └─ └─ relase.yml
├─ git_hooks // set up git hooks for development
├─ ├─ commit-msg
├─ ├─ commit-msg-windows-example
├─ ├─ pre-commit
├─ └─ pre-push
├─ src
├─ ├─ backend //Actix backend, own rust project
├─ ├─ ├─ src
├─ ├─ ├─ ├─ api //Api routes examples
├─ ├─ ├─ ├─ ├─ hello
├─ ├─ ├─ ├─ ├─ ├─ get.rs
├─ ├─ ├─ ├─ ├─ ├─ mod.rs
├─ ├─ ├─ ├─ ├─ └─ post.rs
├─ ├─ ├─ ├─ ├─ space_x // server to server call
├─ ├─ ├─ ├─ ├─ ├─ get.rs
├─ ├─ ├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ ├─ args
├─ ├─ ├─ ├─ ├─ collect_args.rs
├─ ├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ ├─ auth // Simple auth route middleware
├─ ├─ ├─ ├─ ├─ auth_middleware.rs
├─ ├─ ├─ ├─ ├─ login.rs
├─ ├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ ├─ cors
├─ ├─ ├─ ├─ ├─ get_cors_options.rs
├─ ├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ ├─ session // Session middleware examples
├─ ├─ ├─ ├─ ├─ flash_messages.rs
├─ ├─ ├─ ├─ ├─ mod.rs
├─ ├─ ├─ ├─ ├─ session_middleware.rs
├─ ├─ ├─ ├─ └─ validate_session.rs
├─ ├─ ├─ └─ main.rs
├─ ├─ └─ Cargo.toml
├─ ├─ cli // AstroX project runner
├─ ├─ ├─ cmds
├─ ├─ ├─ ├─ tests
├─ ├─ ├─ ├─ ├─ cmd_list_test.rs
├─ ├─ ├─ ├─ ├─ interactive_test.rs
├─ ├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ ├─ cmd_list.rs
├─ ├─ ├─ ├─ execute_cmd.rs
├─ ├─ ├─ ├─ interactive.rs
├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ config
├─ ├─ ├─ ├─ tests
├─ ├─ ├─ ├─ ├─ get_config_test.rs
├─ ├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ ├─ collect_args.rs
├─ ├─ ├─ ├─ create_dotenv.rs
├─ ├─ ├─ ├─ get_config.rs
├─ ├─ ├─ ├─ mod.rs
├─ ├─ ├─ └─ toml.rs
├─ ├─ ├─ development
├─ ├─ ├─ ├─ mod.rs
├─ ├─ ├─ └─ start_development.rs
├─ ├─ ├─ pre_run
├─ ├─ ├─ ├─ cargo
├─ ├─ ├─ ├─ ├─ checks.rs
├─ ├─ ├─ ├─ ├─ mod.rs
├─ ├─ ├─ ├─ └─ validate.rs
├─ ├─ ├─ ├─ npm
├─ ├─ ├─ ├─ ├─ checks.rs
├─ ├─ ├─ ├─ ├─ mod.rs
├─ ├─ ├─ ├─ └─ validate.rs
├─ ├─ ├─ ├─ utils
├─ ├─ ├─ ├─ ├─ check_semver.rs
├─ ├─ ├─ ├─ ├─ git_hooks.rs
├─ ├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ ├─ execute.rs
├─ ├─ ├─ ├─ mod.rs
├─ ├─ ├─ └─ system_checks.rs
├─ ├─ ├─ production
├─ ├─ ├─ ├─ build_production.rs
├─ ├─ ├─ ├─ mod.rs
├─ ├─ ├─ └─ start_production.rs
├─ ├─ ├─ tests
├─ ├─ ├─ ├─ execute.rs
├─ ├─ ├─ └─ mod.rs
├─ ├─ ├─ utils
├─ ├─ ├─ ├─ mod.rs
├─ ├─ ├─ └─ terminal.rs
├─ ├─ └─ mod.rs
├─ ├─ frontend // Astro.Build project
├─ ├─ ├─ .astro
├─ ├─ ├─ └─ settings.json
├─ ├─ ├─ public
├─ ├─ ├─ ├─ astroStation.jpeg
├─ ├─ ├─ ├─ bgAstro.png
├─ ├─ ├─ ├─ bgPattern.png
├─ ├─ ├─ ├─ favicon.svg
├─ ├─ ├─ ├─ hero.jpeg
├─ ├─ ├─ └─ herobc.jpeg
├─ ├─ ├─ src
├─ ├─ ├─ ├─ axiosInstance
├─ ├─ ├─ ├─ ├─ axiosBackendInstance.test.ts
├─ ├─ ├─ ├─ └─ axiosBackendInstance.ts
├─ ├─ ├─ ├─ components
├─ ├─ ├─ ├─ ├─ navbar
├─ ├─ ├─ ├─ ├─ ├─ Navbar.astro
├─ ├─ ├─ ├─ ├─ ├─ Navbar.test.ts
├─ ├─ ├─ ├─ ├─ ├─ NavbarItem.astro
├─ ├─ ├─ ├─ ├─ └─ NavbarItem.test.ts
├─ ├─ ├─ ├─ ├─ spaceX
├─ ├─ ├─ ├─ ├─ ├─ spacex.svelte
├─ ├─ ├─ ├─ ├─ └─ spacex.test.ts
├─ ├─ ├─ ├─ ├─ zoomImage
├─ ├─ ├─ ├─ ├─ ├─ zoomImage.astro
├─ ├─ ├─ ├─ ├─ └─ zoomImage.test.ts
├─ ├─ ├─ ├─ ├─ Card.astro
├─ ├─ ├─ ├─ ├─ Footer.astro
├─ ├─ ├─ ├─ ├─ Hero.astro
├─ ├─ ├─ ├─ └─ Section.astro
├─ ├─ ├─ ├─ layouts
├─ ├─ ├─ ├─ ├─ Layout.astro
├─ ├─ ├─ ├─ └─ Layout.test.ts
├─ ├─ ├─ ├─ pages
├─ ├─ ├─ ├─ ├─ auth
├─ ├─ ├─ ├─ ├─ └─ protected.astro
├─ ├─ ├─ ├─ ├─ 404.astro
├─ ├─ ├─ ├─ ├─ actix.astro
├─ ├─ ├─ ├─ ├─ astro.astro
├─ ├─ ├─ ├─ ├─ cli.astro
├─ ├─ ├─ ├─ └─ index.astro
├─ ├─ ├─ ├─ sections
├─ ├─ ├─ ├─ ├─ Home
├─ ├─ ├─ ├─ ├─ ├─ HomeClone.astro
├─ ├─ ├─ ├─ ├─ ├─ HomeMiddleLinks.astro
├─ ├─ ├─ ├─ ├─ └─ HomeSecondary.astro
├─ ├─ ├─ ├─ └─ imgs
├─ ├─ ├─ ├─ └─ ├─ actix.png
├─ ├─ ├─ ├─ └─ ├─ astro.jpeg
├─ ├─ ├─ ├─ └─ ├─ astro.png
├─ ├─ ├─ ├─ └─ ├─ astro2.jpeg
├─ ├─ ├─ ├─ └─ ├─ cli.png
├─ ├─ ├─ ├─ └─ └─ contact.jpeg
├─ ├─ ├─ ├─ svgs
├─ ├─ ├─ ├─ ├─ Actix.astro
├─ ├─ ├─ ├─ ├─ AstroIcon.astro
├─ ├─ ├─ ├─ ├─ Github.astro
├─ ├─ ├─ ├─ ├─ RustIcon.astro
├─ ├─ ├─ ├─ └─ Spaceout.astro
├─ ├─ ├─ ├─ tests
├─ ├─ ├─ ├─ └─ pages.test.ts
├─ ├─ ├─ └─ env.d.ts
├─ ├─ ├─ .eslintignore
├─ ├─ ├─ .eslintrc.cjs
├─ ├─ ├─ .gitignore
├─ ├─ ├─ .nvmrc
├─ ├─ ├─ astro.config.mjs
├─ ├─ ├─ package.json
├─ ├─ ├─ prettier.config.cjs
├─ ├─ ├─ README.md
├─ ├─ ├─ svelte.config.js
├─ ├─ ├─ tsconfig.json
├─ ├─ └─ vitest.config.ts
├─ └─ main.rs
├─ .gitignore
├─ .nvmrc
├─ Astrox.toml
├─ Cargo.toml
└─ readme.md
```

