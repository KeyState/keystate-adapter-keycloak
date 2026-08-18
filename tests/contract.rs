//! Contract-suite tests: the shared assertions from `keystate-core`'s
//! `testing` module, run against this adapter's manifest and a constructed
//! extraction. These need no live database.

use keystate_adapter_keycloak::{keycloak_manifest, merge_attributes, realm_from_native};
use keystate_core::testing::{
    assert_canonical_deterministic, assert_manifest_valid, assert_serialization_roundtrip,
    assert_verification_complete, assert_volatile_segregation,
};
use keystate_core::version::Version;
use keystate_core::{BackendInfo, CanonicalRealm, EntityKind, ExtractScope};

const SUPPORTED: &str = keystate_adapter_keycloak::SUPPORTED_KEYCLOAK_VERSION;

/// Build a realistic extraction for the supported version, exactly as the
/// extractor would: realm common fields plus a faithful native row.
fn sample_realm() -> CanonicalRealm {
    let version: Version = SUPPORTED.parse().unwrap();
    let native = merge_attributes(
        serde_json::json!({
            "id": "realm-1",
            "name": "master",
            "enabled": true,
            "ssl_required": "EXTERNAL",
            "registration_allowed": false,
        })
        .as_object()
        .unwrap()
        .clone(),
        vec![("theme".into(), "keystore".into())],
    );
    CanonicalRealm {
        backend: BackendInfo {
            backend: "keycloak".into(),
            detected_version: version,
            extra: Default::default(),
        },
        realm: realm_from_native("master", native),
        ..Default::default()
    }
}

#[test]
fn manifest_is_self_consistent() {
    let version: Version = SUPPORTED.parse().unwrap();
    let manifest = keycloak_manifest(&version).expect("supported version has a manifest");
    assert_manifest_valid(&manifest);
}

#[test]
fn extracted_realm_is_deterministic_and_roundtrips() {
    let realm = sample_realm();
    assert_canonical_deterministic(&realm);
    assert_serialization_roundtrip(&realm);
}

#[test]
fn extraction_verifies_complete_against_manifest() {
    let version: Version = SUPPORTED.parse().unwrap();
    let manifest = keycloak_manifest(&version).unwrap();
    let realm = sample_realm();
    assert_verification_complete(&realm, &manifest);
}

#[test]
fn volatile_segregation_holds() {
    let version: Version = SUPPORTED.parse().unwrap();
    let manifest = keycloak_manifest(&version).unwrap();
    let realm = sample_realm();
    assert_volatile_segregation(&realm, &manifest);
}

#[test]
fn scope_roundtrips() {
    assert_serialization_roundtrip(&ExtractScope::new("master"));
}

#[test]
fn entity_kind_realm_is_identified_by_name() {
    assert_eq!(EntityKind::Realm.id_field(), "name");
}
