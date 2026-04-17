# Contributing to kite-protocol

This crate defines the wire format shared by every Kite component (server, CLI, mesh daemon). A change here ripples out to every consumer, so be deliberate.

## Local development

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
cargo publish --dry-run
```

## Changing the wire format

- **Additive changes** (new optional field, new variant in a non-exhaustive enum): patch release.
- **Breaking changes** (renamed field, removed variant, changed serialization): bump minor (pre-1.0) and coordinate with server + CLI bumps in the same week.

When you change Rust types, regenerate the TypeScript bindings and commit them alongside:

```bash
cargo test --features ts-bindings  # produces files in bindings/
```

## Release process

1. Bump `version` in `Cargo.toml`.
2. Open a PR; CI runs fmt/clippy/test/`cargo publish --dry-run`.
3. Merge to `main`; tag `vX.Y.Z`.
4. The `publish.yml` workflow publishes to crates.io using `CRATES_IO_TOKEN`.

## License

By contributing you agree your contributions are licensed under MIT.
