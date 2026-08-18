//! Realm-level field manifest for the supported Keycloak version.

use keystate_core::EntityKind;
use keystate_core::verify::{EntityExpectation, EntityPresence, FieldExpectation, FieldManifest};
use keystate_core::version::Version;

use crate::db::version::SUPPORTED_KEYCLOAK_VERSION;

/// A required, non-volatile field expectation for `Realm` rows.
fn realm_field(path: &str, introduced_in: Version) -> FieldExpectation {
    FieldExpectation {
        entity: EntityKind::Realm,
        path: path.to_string(),
        required: true,
        volatile: false,
        introduced_in,
    }
}

/// Field manifest covering the `Realm` entity for the supported version.
///
/// Only fields this adapter actually extracts and verifies are listed.
/// Additive-only: a field added in a later Keycloak version is a new
/// expectation here, never a silent gap.
pub fn realm_manifest(version: &Version) -> Option<FieldManifest> {
    if version.to_string() != SUPPORTED_KEYCLOAK_VERSION {
        return None;
    }
    let introduced = SUPPORTED_KEYCLOAK_VERSION.parse().expect("constant parses");
    let mut manifest = FieldManifest::new("keycloak", *version);
    manifest.entities.push(EntityExpectation {
        entity: EntityKind::Realm,
        presence: EntityPresence::Required,
    });
    manifest.fields = vec![
        realm_field("name", introduced),
        realm_field("enabled", introduced),
        realm_field("native.id", introduced),
    ];
    Some(manifest)
}
