# kite-protocol

[![Crates.io](https://img.shields.io/crates/v/kite-protocol.svg)](https://crates.io/crates/kite-protocol)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

Shared wire-format types for the [Kite](https://github.com/Alpha-Centauri-Cyberspace) webhook delivery protocol and the forthcoming [Kite Mesh](https://github.com/Alpha-Centauri-Cyberspace/kite-mesh).

This crate is the single source of truth for:

- CloudEvents-based agent messages exchanged between the Kite CLI and the Kite relay server
- WebSocket message framing
- Event extensions for Kite-specific metadata

Both the [kite-cli](https://github.com/Alpha-Centauri-Cyberspace/kite-cli) and the private kite-server consume this crate. TypeScript bindings are generated via [`ts-rs`](https://crates.io/crates/ts-rs) and checked into the `bindings/` directory.

## Use

```toml
[dependencies]
kite-protocol = "0.1"
```

## Versioning

Wire-format breaking changes bump the minor version until 1.0. Patch releases are fully backwards compatible on the wire.

## Contributing

PRs to this repo are how the Kite wire format evolves — server, CLI, and mesh daemons all pin against published versions. See [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

MIT — see [LICENSE](./LICENSE).
