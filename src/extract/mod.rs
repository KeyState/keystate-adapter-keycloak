//! Per-entity extraction for Keycloak.
//!
//! One module per canonical entity. Each module knows how to pull that
//! entity's rows out of Keycloak's PostgreSQL schema and build the canonical
//! row — the small set of *common* fields plus a backend-faithful `native`
//! representation — sorted by a stable key, exactly as the core contract
//! requires.
//!
//! The orchestrator [`KeycloakExtractor`] implements
//! [`keystate_core::port::Extractor`], driving the per-entity modules in a
//! fixed order.

pub mod realm;

use keystate_core::model::{BackendInfo, CanonicalRealm, ExtractScope};
use keystate_core::port::Extractor;
use keystate_core::version::Version;

use crate::db::KeycloakConnection;
use crate::db::error::Result;
use crate::db::version::{SUPPORTED_KEYCLOAK_VERSION, detect_keycloak_version, ensure_supported};

/// The Keycloak extraction adapter, implementing core's [`Extractor`] port.
///
/// Holds a read-only connection pool. Construction does not touch the
/// network; connection happens lazily on the first query.
#[derive(Debug, Clone)]
pub struct KeycloakExtractor {
    connection: KeycloakConnection,
}

impl KeycloakExtractor {
    /// Build an extractor for the Keycloak database at `database_url`.
    ///
    /// The pool connects lazily on first use, so construction is synchronous
    /// and cannot fail; connection errors surface on the first query.
    pub fn new(database_url: impl Into<String>) -> Result<Self> {
        Ok(Self {
            connection: KeycloakConnection::connect_lazy(&database_url.into())?,
        })
    }
}

impl Extractor for KeycloakExtractor {
    async fn detect(&self) -> keystate_core::Result<BackendInfo> {
        let version: Version = detect_keycloak_version(self.connection.pool()).await?;
        ensure_supported(&version)?;
        Ok(BackendInfo {
            backend: "keycloak".into(),
            detected_version: version,
            extra: Default::default(),
        })
    }

    async fn extract(&self, scope: &ExtractScope) -> keystate_core::Result<CanonicalRealm> {
        // Detect again so the manifest is bound to the version actually
        // present at extraction time, not a value captured at construction.
        let version: Version = detect_keycloak_version(self.connection.pool()).await?;
        ensure_supported(&version)?;

        let realm = realm::extract_realm(self.connection.pool(), &scope.realm).await?;

        Ok(CanonicalRealm {
            backend: BackendInfo {
                backend: "keycloak".into(),
                detected_version: version,
                extra: Default::default(),
            },
            realm,
            ..Default::default()
        })
    }
}

/// The version this adapter is built and validated against.
pub const SUPPORTED_VERSION: &str = SUPPORTED_KEYCLOAK_VERSION;

// Re-export the realm mapping helpers so the contract suite in `tests/` can
// build deterministic fixtures without a live database.
pub use realm::{merge_attributes, realm_from_native};
