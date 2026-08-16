//! Realm extraction for Keycloak.
//!
//! Pulls one realm's row from the `REALM` table, maps its identity key and
//! enabled state into the canonical common fields, and keeps the *entire*
//! backend row faithfully in `native` — field-for-field, as Keycloak itself
//! names it — so nothing is silently dropped. The realm is identified by its
//! `NAME`, which Keycloak guarantees unique.

use keystate_core::model::Realm;
use serde_json::{Map, Value};
use sqlx::PgPool;

use crate::db::error::{Error, Result};

/// Selects the entire realm row as a JSON object.
///
/// `row_to_json` returns Keycloak's own column names and types verbatim, so
/// the native representation is exactly what the backend stores — the basis
/// for backend-faithful round-trips and for comparing two extractions of the
/// same realm (they must be byte-identical).
const REALM_QUERY: &str = r#"
    SELECT row_to_json(r) AS native
    FROM realm r
    WHERE r.name = $1
    LIMIT 1
"#;

/// Extract one realm, identified by `name`.
pub async fn extract_realm(pool: &PgPool, name: &str) -> Result<Realm> {
    let native: Option<Value> = sqlx::query_scalar(REALM_QUERY)
        .bind(name)
        .fetch_optional(pool)
        .await?;

    let Some(native) = native else {
        return Err(Error::RealmNotFound { realm: name.into() });
    };

    Ok(realm_from_native(name, native))
}

/// Map a Keycloak realm row (as JSON) into the canonical [`Realm`].
///
/// The common fields (`name`, `enabled`) are read from the backend-faithful
/// row; the whole row is preserved as `native`. This is a pure function so
/// the mapping is unit-testable without a database.
pub fn realm_from_native(name: &str, native: Value) -> Realm {
    let enabled = native
        .get("enabled")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    Realm {
        name: name.to_string(),
        enabled,
        native,
    }
}

/// Merge realm-level attributes from `REALM_ATTRIBUTES` into the native
/// object under a dedicated key, so custom realm settings survive.
///
/// Not yet wired into [`extract_realm`]; reserved for the moment the
/// attributes table's exact shape for the supported version is confirmed
/// against a live instance (see `RELEASE.md` §2).
pub fn merge_attributes(
    mut native: Map<String, Value>,
    attributes: Vec<(String, String)>,
) -> Value {
    let attrs: Map<String, Value> = attributes
        .into_iter()
        .map(|(k, v)| (k, Value::String(v)))
        .collect();
    if !attrs.is_empty() {
        native.insert("realm_attributes".into(), Value::Object(attrs));
    }
    Value::Object(native)
}
