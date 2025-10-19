# AGENTS

## Purpose

This document describes how automated agents should interact with this repository. Follow every instruction unless the user explicitly overrides it.

## Quickstart

0. If this project was regenerated, choose a `project_name` with alphanumerics so the derived slug remains valid.
1. Run `mise trust` followed by `mise install` to provision local tools.
2. Use `mise run fmt`, `mise run lint`, and `mise run test` before opening a pull request.
3. Execute work in a red–green–refactor loop: write a failing test, make it pass, then tidy the code.
4. Never commit generated artefacts from `coverage/` or `target/`.
5. Stay on the pinned Rust toolchain (`rust-toolchain.toml`) to guarantee edition 2024 compatibility and MSRV 1.85 features.

## Tooling

- Tool versions are pinned in `mise.toml`. Do not install tools globally.
- `rust-toolchain.toml` keeps `rustfmt`, `clippy`, and `llvm-tools-preview` aligned at Rust 1.85.1 (edition 2024). Avoid overriding the toolchain without discussion.
- `cargo-nextest` powers the primary test task. Prefer it over `cargo test` unless you need doctests.
- `cargo-llvm-cov` generates coverage metrics. Output lives in `coverage/`.

## Tasks

Run tasks with `mise run <task>`:

- `fmt` — format the workspace with `cargo fmt --all`.
- `lint` — clippy (pedantic, warnings-as-errors), typos, markdownlint.
- `lint-fix` — auto-fix typos and Markdown issues, then format code.
- `test` — execute the nextest suite across the workspace.
- `test-doc` — run `cargo test --doc` for doctests.
- `coverage` — produce lcov and HTML reports via llvm-cov.
- `pkg-install` — install the binary locally using `cargo install --path .`.
- `check` — aggregated guard that depends on `fmt`, `lint`, `test`, and `test-doc`.

## Coding Standards

- Keep modules and functions small; avoid speculative abstractions.
- Return rich errors using `color-eyre`'s `Result` alias for consistent diagnostics.
- Handle clap subcommands in `src/lib.rs`; keep `src/main.rs` minimal.
- Public functions that return `Result` must explain failure modes in doc comments.
- Do not introduce new global state or singletons.
- Prefer adjusting lint levels via the `[lints.clippy]` table in `Cargo.toml` rather than sprinkling `#[allow]` attributes.

## Testing & Coverage

- Add tests under `tests/` using `assert_cmd` to exercise CLI surface area.
- When adding new behaviour to `src/lib.rs`, prefer unit tests located alongside the module.
- Keep fixtures under `tests/fixtures/` (create the directory when needed).
- Generate coverage with `mise run coverage`; upload artefacts in CI if required by downstream tooling.

## Continuous Integration

- Workflows use `jdx/mise-action` to sync tool versions. Avoid adding ad-hoc install steps.
- Jobs run `mise run fmt`, `mise run lint`, `mise run test`, and `mise run coverage`. Keep tasks deterministic and idempotent.
- Update caching configuration rather than skipping tasks if build time increases.

## Releases

- `cargo dist` configuration is user-managed. Run `cargo dist init` when you are ready to automate releases, then commit the generated files.
- Document each release in `CHANGELOG.md` following Keep a Changelog conventions.

## Housekeeping

- Update `typos.toml` when introducing new proper nouns to prevent CI failures.
- Keep `rust-toolchain.toml` and `Cargo.toml` in sync when bumping the MSRV.
- Confirm `AGENTS.md` stays current whenever tooling changes.

## Escalations

If a task requires new tooling or significant workflow changes, open an issue tagged `workflow` and document the proposal before implementing.
