//! Database access for the Keycloak adapter.
//!
//! Everything that speaks PostgreSQL lives here and nowhere else: the
//! read-only connection, schema-version detection, and the error type the
//! database layer surfaces. The rest of the crate is pure mapping and
//! manifest logic, so the boundary between "talks to Keycloak's database"
//! and "shapes data" stays explicit.
//!
//! Module map:
//!
//! - [`connection`] — pooled, read-only Postgres connection.
//! - [`version`] — schema-version detection and the supported-version gate.
//! - [`error`] — database-layer error and result types.

pub mod connection;
pub mod error;
pub mod version;

pub use connection::KeycloakConnection;
pub use error::{Error, Result};
pub use version::{SUPPORTED_KEYCLOAK_VERSION, detect_keycloak_version, ensure_supported};
