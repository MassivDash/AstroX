# Workspace Rules & Verification Pipeline

## Test-Driven Development (TDD)
You MUST follow a Test-Driven Development (TDD) approach when developing features or fixing bugs:
1. **Write a failing test** first that defines the expected behavior of the new feature or describes the bug.
2. **Implement the minimum code** required to make the test pass.
3. **Refactor** the code while ensuring all tests continue to pass.

## Core Principles (YAGNI & KISS)
- **KISS (Keep It Simple, Stupid)**: Avoid over-engineering. Favor readable, direct, and simple code over complex or overly clever abstractions.
- **YAGNI ("We will not need that in the future")**: Do not write code or add hooks under the assumption that they might be useful later. Only implement what is strictly necessary to satisfy the current requirements.

## Verification Pipeline
After completing any feature, bug fix, or codebase modification, you MUST run the verification pipeline to ensure everything is building, formatted, linted, and tested. Set `set -e` to exit immediately if any command fails.

Run the following commands in the workspace root, in this exact order, every time you finish a piece of work (a feature, a fix, or any codebase modification) — before reporting the task as done:

```bash
set -e

echo '🧪 running cli cargo test...'
cargo test

echo '🧹 running cargo fmt...'
cargo fmt

echo '🔍 running cargo clippy...'
cargo clippy -- -D warnings

echo '🧪 running backend cargo test...'
cargo test --manifest-path src/backend/Cargo.toml

echo '🧹 running backend cargo fmt...'
cargo fmt --manifest-path src/backend/Cargo.toml

echo '🔍 running backend cargo clippy...'
cargo clippy --manifest-path src/backend/Cargo.toml -- -D warnings

echo '🧹 running frontend project lint...'
npm run lint:staged --prefix src/frontend/

echo '🧪 running related frontend vitest...'
npm run test:related --prefix src/frontend/

echo '----- Done -----'
```

Notes:
- `set -e` means the script stops at the first failing command — fix it and rerun the whole pipeline from the top, don't skip ahead.
- `lint:staged` only checks files that are `git add`ed (staged); `git add` the files you touched before running the pipeline, or this step will silently pass without checking anything.
- `test:related` (`vitest related --run`) only runs tests related to changed files, not the full suite — it relies on the working tree diff, so run it before staging is strictly necessary but after your edits are saved.

Do not mark the task as complete or present the changes for final review until all verification steps execute successfully with zero warnings/errors.
