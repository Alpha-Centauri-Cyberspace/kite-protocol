<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://getkite.sh/logo-on-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://getkite.sh/logo-on-light.svg">
    <img alt="Kite" src="https://getkite.sh/logo-on-dark.svg" width="220">
  </picture>

  <h3>Event delivery for developers and AI agents</h3>

  <p>
    <a href="https://crates.io/crates/kite-protocol"><img alt="crates.io" src="https://img.shields.io/crates/v/kite-protocol?color=00ff9d&labelColor=0a0a0f&style=flat-square"></a>
    <a href="https://docs.rs/kite-protocol"><img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-kite--protocol-00d4ff?style=flat-square&labelColor=0a0a0f"></a>
    <a href="https://getkite.sh"><img alt="Website" src="https://img.shields.io/badge/getkite.sh-00ff9d?style=flat-square&labelColor=0a0a0f"></a>
    <a href="./LICENSE"><img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-e4e4e7?style=flat-square&labelColor=0a0a0f"></a>
  </p>
</div>

---

`kite-protocol` defines the shared Kite wire-format types used by the CLI and private relay server. Third-party clients can use this crate to construct and parse protocol messages.

## Use

```toml
[dependencies]
kite-protocol = "0.1"
```

This constructs a handshake message without opening a connection. Replace the placeholder API key and team ID with your configured values before connecting.

```rust
use kite_protocol::ClientMessage;

let api_key = "your-api-key".to_owned();
let team_id = "your-team-id".to_owned();

let connect = ClientMessage::Connect {
    version: 1,
    token: api_key,
    team_id,
    scopes: vec!["source:github".into()],
    client_id: None,
};
```

## What's inside

- **WebSocket framing** — `ClientMessage` and `ServerMessage` enums covering `connect`, `request`, `event`, `response`, `error`, `quota_snapshot`, and `billing_block`.
- **CloudEvents extensions** — helpers for `kiteseq`, `kitesummary`, `kiteoriginalheaders`, and federation metadata on top of [CloudEvents v1.0](https://cloudevents.io).
- **Agent messages** — payload types and helpers for `com.kite.agent.message` events.
- **TypeScript bindings** — auto-generated via [`ts-rs`](https://crates.io/crates/ts-rs) and checked into [`bindings/`](./bindings) for downstream TS consumers.

## Versioning

Pre-1.0: wire-format breaking changes require a **minor version bump**; patches are wire-compatible. Breaking changes need a coordinated release across affected consumers.

If you're building a third-party agent, pin to `"0.1"` and track release notes.

## Consumers

- **[kite-cli](https://github.com/Alpha-Centauri-Cyberspace/kite-cli)** — the universal webhook adapter CLI.
- **Kite server** — the relay at `api.getkite.sh` (private).

For mesh-layer development, see [kite-mesh](https://github.com/Alpha-Centauri-Cyberspace/kite-mesh).

## Contributing

PRs against this repo evolve the wire format for the entire Kite ecosystem. See [`CONTRIBUTING.md`](./CONTRIBUTING.md). Changes that break compatibility need a version bump and a synchronized release across all consumers — please flag them in the PR description.

## License

MIT — see [`LICENSE`](./LICENSE).

---

<div align="center">
  <sub>
    <a href="https://getkite.sh">getkite.sh</a> ·
    <a href="https://github.com/Alpha-Centauri-Cyberspace">github</a> ·
    <a href="https://getkite.sh/docs">docs</a>
  </sub>
</div>
