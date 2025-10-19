# Release Process

This repository does not include a pre-generated `dist-workspace.toml`. The `cargo dist` tool manages that file and its format may change between releases. Follow these steps when you are ready to automate binary distribution:

1. Ensure a clean working tree and up-to-date `CHANGELOG.md` entry.
2. Install dist tooling:
   ```bash
   cargo install cargo-dist
   ```
3. Initialize the configuration:
   ```bash
   cargo dist init
   ```
4. Review the generated `dist-workspace.toml` and commit it alongside any workflow updates that `cargo dist` suggests.
5. For each release candidate:
   ```bash
   cargo dist plan
   cargo dist build
   cargo dist host
   cargo dist announce
   ```
6. Tag the release and push to GitHub.

If cargo-dist modifies workflow files, prefer running it locally and committing the diff rather than editing manually.
