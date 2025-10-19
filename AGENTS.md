# AGENTS

## Purpose

This repository contains the Cookiecutter template used to scaffold Rust CLI projects with standardized tooling (mise, cargo-nextest, cargo-llvm-cov, typos, markdownlint-cli2, and clippy `pedantic`).

## Workflow

- Generate a project via `cookiecutter /path/to/rust-cli`. The only required answers are those documented in `README.md`; all other identifiers are derived automatically.
- Accept the default slug/crate/binary prompts unless you have a strong reason to deviate—this keeps naming consistent.
- After generating a project, follow the generated `README.md` instructions (`mise trust`, `mise install`, `mise run check`).
- The template targets Rust edition 2024 with an MSRV of 1.90; keep downstream projects on a compatible toolchain unless explicitly discussed.
- The template intentionally includes an example `greet` command and integration tests to ensure immediate test coverage. Downstream projects can remove or replace them.

## Editing Guidance

- Modify files under `{{cookiecutter.project_slug}}/` using Jinja templating. Keep derived identifiers (`project_slug`, `crate_name`, etc.) consistent across files.
- When changing tooling (e.g., adding tasks or linters), update both `mise.toml` and the generated `AGENTS.md` so new projects inherit the new workflow.
- Validation hooks have been removed; rely on documentation and generated project builds to surface bad inputs. Document naming expectations clearly.
- Prefer small, isolated commits when adjusting template logic to keep diffs easy to review.

## Quality Checks

- After any template change, generate a sample project (`cookiecutter . --no-input`) and run:
  - `cargo fmt --all`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`
- Confirm `README.md` and the generated project's docs mention the naming requirements for `project_name` (must contain alphanumeric characters so slugs stay valid).
