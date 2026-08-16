# keystate-adapter-keycloak

**Keystate adapter** that extracts a Keycloak realm's complete configuration
state directly out of its PostgreSQL database and maps it into the canonical
model defined by [`keystate-core`](https://github.com/KeyState/keystate-core).

This crate is **one backend, one job**: it knows Keycloak's schema and nothing
else. The canonical model, the completeness-verification engine, and the
output ports all live in `keystate-core`.

## Status

In development — **v0.2.0: first extraction (Realm only)**. It extracts the
`Realm` entity, detects the Keycloak schema version, and verifies completeness
against a per-version field manifest. Client, user, and flow extraction arrive
in later roadmap releases (`ignore-ROADMAP.md`).

## Supported Keycloak version

This adapter targets **exactly one** Keycloak version at a time — the current
stable when the release was cut (see `SUPPORTED_KEYCLOAK_VERSION` in
`src/version.rs`). Extraction and verification refuse to proceed against an
unsupported schema. Supporting other versions is a separate, paid maintenance
effort, not part of this release.

## Module map

| Module            | Responsibility                                                        |
| ----------------- | --------------------------------------------------------------------- |
| `connection`      | Read-only Postgres connection with Keystate session defaults          |
| `version`         | Keycloak schema version detection and support gate                    |
| `extract::realm`  | Realm row → canonical `Realm` (common fields + faithful `native`)     |
| `extract`         | [`Extractor`] orchestrator implementing the core port                 |
| `manifest::realm` | Realm field manifest for the supported version                        |
| `error`           | Adapter error type wrapping `keystate-core`'s                         |

Every module is one concern, so adding an entity (client, user, …) means adding
one new file under `extract/` and one under `manifest/`, reusing the existing
orchestrator.

## Design principles

- **Backend-faithful**: the whole realm row is preserved in `native` exactly
  as Keycloak stores it; nothing is silently dropped.
- **Read-only**: the session is forced `default_transaction_read_only=on` —
  the adapter cannot modify a live Keycloak database.
- **Version-bound**: the manifest and report are keyed to the schema version
  actually detected at the source, never assumed.
- **Deterministic**: extraction produces byte-stable output (contract-suite
  enforced) so two runs over the same realm diff cleanly.

## Development

See `DEVELOPMENT.md`. Prerequisites: Rust 1.85+.

```sh
cargo build
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## License

Apache-2.0. See `LICENSE`.