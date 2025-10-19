# {{ cookiecutter.project_name | title() }}

{{ cookiecutter.description }}

## Installation

Install via `cargo`:

```bash
cargo install --git https://github.com/{{ cookiecutter.github_handle }}/{{ cookiecutter.project_slug }} {{ cookiecutter.binary_name }}
```

Or install locally as a mise tool:

```bash
mise run pkg-install
```

## Usage

Show CLI help:

```bash
{{ cookiecutter.binary_name }} --help
```

Try the example command:

```bash
{{ cookiecutter.binary_name }} greet --name you
```

## Development Workflow

```bash
mise trust
mise install
```

This project targets Rust edition 2024. The toolchain is pinned to {{ cookiecutter.rust_toolchain }} in `rust-toolchain.toml`, which satisfies the 1.85 minimum for the edition.

### Common Tasks

- `mise run fmt`
- `mise run lint`
- `mise run test`
- `mise run coverage`
- `mise run test-doc`

### Red-Green-Refactor Cycle

1. Add or update a test in `tests/` that expresses the new behaviour.
2. `mise run test` to watch the failure.
3. Implement the smallest change in `src/` to make the test pass.
4. Clean up before committing (`mise run fmt` and `mise run lint`).

### Coverage

Ensure the `llvm-tools-preview` component is installed (handled by `rust-toolchain.toml`).
Generate an lcov report and HTML output:

```bash
mise run coverage
open coverage/html/index.html
```

### Releasing

Follow `docs/releasing.md` for manual `cargo dist` setup and release automation guidance.

## Contributing

Open an issue or pull request on GitHub at `https://github.com/{{ cookiecutter.github_handle }}/{{ cookiecutter.project_slug }}`.
