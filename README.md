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

`kite-protocol` is the single source of truth for the Kite wire format. Every client and server in the Kite ecosystem pins against this crate — the CLI, the relay server, the mesh daemon, and any third-party agent that wants to speak the protocol directly.

## Use

```toml
[dependencies]
kite-protocol = "0.1"
```

```rust
use kite_protocol::{ClientMessage, ServerMessage};

let connect = ClientMessage::Connect {
    version: "0.1".into(),
    token: api_key,
    team_id,
    scopes: vec!["events.read".into()],
};
```

## What's inside

- **WebSocket framing** — `ClientMessage` and `ServerMessage` enums covering `connect`, `request`, `event`, `response`, `error`, `quota_snapshot`, and `billing_block`.
- **CloudEvents extensions** — Kite-specific metadata on top of [CloudEvents v1.0](https://cloudevents.io): `team_id`, `source`, `importance`, `signature`, delivery cursors.
- **Signature helpers** — HMAC-SHA256 verification for outbound webhook signatures.
- **TypeScript bindings** — auto-generated via [`ts-rs`](https://crates.io/crates/ts-rs) and checked into [`bindings/`](./bindings) for downstream TS consumers.

## Versioning

Pre-1.0: **minor bumps are breaking** on the wire; patches are fully wire-compatible. Every consumer (kite-cli, the Kite server, kite-mesh) pins a compatible minor and ships a coordinated release when this crate rolls.

If you're building a third-party agent, pin to `"0.1"` and track release notes.

## Consumers

- **[kite-cli](https://github.com/Alpha-Centauri-Cyberspace/kite-cli)** — the universal webhook adapter CLI.
- **[kite-mesh](https://github.com/Alpha-Centauri-Cyberspace/kite-mesh)** — P2P capability discovery for AI agents.
- **Kite server** — the relay at `api.getkite.sh` (private).

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
