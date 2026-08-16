//! # keystate-adapter-keycloak
//!
//! Keycloak database extraction adapter for **Keystate** — the concrete
//! [`keystate_core::port::Extractor`] implementation that reads a Keycloak
//! realm's configuration state directly out of its PostgreSQL database and
//! maps it into the canonical model defined by
//! [`keystate_core`](https://github.com/KeyState/keystate-core).
//!
//! This crate is deliberately **one backend, one job**: it knows Keycloak's
//! schema and nothing else. The canonical model, verification engine, and
//! output ports all live in `keystate-core`; this crate only implements the
//! [`keystate_core::port::Extractor`] and
//! [`keystate_core::port::CompletenessVerifier`] ports against Keycloak's
//! database.
//!
//! # Module map
//!
//! - [`db`] — everything that speaks PostgreSQL: the read-only connection,
//!   schema-version detection, and the database-layer error type.
//! - [`extract`] — per-entity extraction: one module per canonical entity,
//!   returning native JSON faithful to Keycloak's own field shape.
//! - [`manifest`] — per-version field manifests describing what a given
//!   Keycloak version must expose.
//!
//! See `ARCHITECTURE.md` in this repository for the full design, and the
//! companion `keystate-core` crate for the ports and canonical model.

#![warn(missing_docs)]

pub mod db;
pub mod extract;
pub mod manifest;

pub use db::{KeycloakConnection, SUPPORTED_KEYCLOAK_VERSION, detect_keycloak_version};
pub use extract::{KeycloakExtractor, merge_attributes, realm_from_native};
pub use manifest::keycloak_manifest;

/// The adapter's database-layer error type.
pub use db::Error;
/// The adapter's database-layer result alias.
pub use db::Result;
