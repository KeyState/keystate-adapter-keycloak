//! Keycloak schema version detection.
//!
//! Keystate verifies completeness **against the version actually present in
//! the database**, so the first thing an extraction needs is the Keycloak
//! schema version. This is read from Keycloak's own migration bookkeeping
//! (`MIGRATION_MODEL`, which records the highest applied migration version),
//! parsed into [`Version`] by `keystate-core`.

use keystate_core::version::Version;
use sqlx::PgPool;

use crate::db::error::{Error, Result};

/// Query returning the highest applied schema version, e.g. `26.5.5`.
///
/// Keycloak records every applied migration in `MIGRATION_MODEL` with a
/// monotonically increasing `UPDATE_TIME`, so the most recent row is the
/// current schema version.
const VERSION_QUERY: &str = r#"
    SELECT version
    FROM migration_model
    ORDER BY update_time DESC
    LIMIT 1
"#;

/// The single Keycloak version this adapter supports.
///
/// Per the project's release model, this adapter targets exactly one Keycloak
/// version at a time — the current stable at the moment the release was cut.
/// Supporting additional versions is a separate, paid maintenance effort, not
/// part of the core release. See `RELEASE.md` §2 in this repository.
pub const SUPPORTED_KEYCLOAK_VERSION: &str = "26.5.5";

/// Detect the Keycloak schema version present in the database.
///
/// Returns the version as a [`Version`], ready to key the field manifest and
/// the verification report.
pub async fn detect_keycloak_version(pool: &PgPool) -> Result<Version> {
    let row: Option<(String,)> = sqlx::query_as(VERSION_QUERY).fetch_optional(pool).await?;
    let version = row
        .map(|(v,)| v)
        .ok_or_else(|| Error::Message("Keycloak database has no migration model".into()))?;
    Ok(version.parse()?)
}

/// Check a detected version against the supported one.
///
/// This is the enforcement point for "current version only": extraction and
/// verification refuse to proceed against an unsupported schema, so a user
/// can never get a completeness report for a Keycloak version this adapter
/// wasn't built and validated against.
pub fn ensure_supported(detected: &Version) -> Result<()> {
    let supported: Version = SUPPORTED_KEYCLOAK_VERSION.parse()?;
    if detected == &supported {
        Ok(())
    } else {
        Err(Error::UnsupportedVersion {
            detected: detected.to_string(),
            supported: SUPPORTED_KEYCLOAK_VERSION.to_string(),
        })
    }
}
