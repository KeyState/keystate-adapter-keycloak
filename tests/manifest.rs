//! Manifest tests: presence for the supported version, absence otherwise.

use keystate_adapter_keycloak::keycloak_manifest;
use keystate_core::version::Version;

const SUPPORTED: &str = keystate_adapter_keycloak::SUPPORTED_KEYCLOAK_VERSION;

#[test]
fn manifest_exists_for_supported_version() {
    let v: Version = SUPPORTED.parse().unwrap();
    let manifest = keycloak_manifest(&v).expect("supported version has a manifest");
    assert!(manifest.validate().is_ok());
    assert_eq!(manifest.fields.len(), 3);
    assert_eq!(manifest.entities.len(), 1);
    assert_eq!(manifest.backend, "keycloak");
}

#[test]
fn no_manifest_for_unsupported_version() {
    assert!(keycloak_manifest(&Version::new(25, 0, 0)).is_none());
    assert!(keycloak_manifest(&Version::new(27, 0, 0)).is_none());
}

#[test]
fn manifest_roundtrips() {
    let v: Version = SUPPORTED.parse().unwrap();
    let manifest = keycloak_manifest(&v).unwrap();
    let bytes = keystate_core::canonical_bytes(&manifest).expect("canonical bytes");
    let back: keystate_core::verify::FieldManifest =
        serde_json::from_slice(&bytes).expect("deserialize");
    assert_eq!(back, manifest);
}
