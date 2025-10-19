# rust-cli

Cookiecutter template for building opinionated Rust command-line applications with mise-managed tooling, clippy `pedantic`, cargo-nextest, cargo-llvm-cov, typos, and markdownlint-cli2 baked in.

## Highlights

- MIT-licensed scaffold tailored for [@bedecarroll](https://github.com/bedecarroll).
- Zero-config mise tasks for fmt, lint, tests, doctests, coverage, and local install.
- Clap-based CLI entrypoint with color-eyre error handling and assert_cmd integration tests.
- Documentation-first workflow via `README.md`, `AGENTS.md`, `CHANGELOG.md`, and `docs/releasing.md`.
- GitHub Actions CI that reuses mise tasks and keeps coverage publishing on Linux pushes.
- Rust 2024 edition by default with MSRV pinned to Rust 1.90 for modern language features.

## Usage

```bash
cookiecutter gh:bedecarroll/rust-cli
```

Answer the prompts (defaults are tuned for quick starts). After generation:

```bash
cd <project>
mise trust
mise install
mise run check
mise run coverage  # optional, requires llvm tools
```

## Template Variables

- `project_name`: Human-readable project name (kebab-case slug and crate name are derived).
- `project_slug`: Derived repository/crate slug (press Enter to accept the default).
- `crate_name`: Derived Rust crate identifier (press Enter to accept the default).
- `binary_name`: Derived binary name (press Enter to accept the default).
- `description`: Short sentence for Cargo metadata.
- `rust_toolchain`: Toolchain channel (defaults to `1.90.0`).
- `rust_msrv`: Minimum supported Rust version (defaults to `1.90` for Rust 2024 edition).
- `license_year`: Copyright year stamped into `LICENSE`.

Cookiecutter will prompt for derived values (slug, crate, binary); press Enter to accept the defaults so naming stays consistent.

`project_name` should include alphanumeric characters so the derived slug (`project-name`) and crate identifier (`project_name`) remain valid Rust identifiers. If the generated names do not compile, regenerate with a different `project_name`.

## What You Get

- `Cargo.toml` preset with clap, color-eyre, assert_cmd.
- `mise.toml` pinning rust, cargo-nextest, cargo-llvm-cov, typos-cli, markdownlint-cli2.
- `.config/nextest.toml` with CI profile and junit output.
- `[lints.clippy]` configuration enabling `clippy::pedantic` globally.
- `typos.toml` and `.markdownlint-cli2.yaml` for consistent linting.
- `AGENTS.md` describing workflows for automation and contributors.
- `LICENSE` prefilled with MIT boilerplate.
- `docs/releasing.md` with guidance to run `cargo dist init` manually.
- Example `greet` subcommand plus integration tests for immediate red-green.
- GitHub Actions workflow leveraging `jdx/mise-action`.

## Extending

- Update `mise.toml` when new tasks or tools are required; keep commands idempotent for CI reuse.
- Tune lint levels in `Cargo.toml`'s `[lints.clippy]` table if specific checks need to be relaxed (prefer targeted `allow` attributes).
- Add extra docs under `docs/` as conventions evolve, ensuring `AGENTS.md` stays in sync.
- When distribution is needed, run `cargo dist init` and commit the generated files per `docs/releasing.md`.

## Inspiration

- [Typer cookiecutter](https://github.com/bedecarroll/typer-app)
- [minijinja](https://github.com/mitsuhiko/minijinja)
- [ruff](https://github.com/astral-sh/ruff)
