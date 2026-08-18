//! Pure mapping tests for realm extraction — no live database required.

use keystate_adapter_keycloak::{merge_attributes, realm_from_native};

#[test]
fn maps_common_fields_and_keeps_native() {
    let native = serde_json::json!({
        "id": "a1b2c3",
        "name": "master",
        "enabled": true,
        "ssl_required": "EXTERNAL",
        "registration_allowed": false,
    });
    let realm = realm_from_native("master", native.clone());
    assert_eq!(realm.name, "master");
    assert!(realm.enabled);
    assert_eq!(realm.native, native);
}

#[test]
fn enabled_defaults_to_false_when_absent() {
    let realm = realm_from_native("master", serde_json::json!({ "name": "master" }));
    assert!(!realm.enabled);
}

#[test]
fn merge_attributes_are_preserved_under_a_dedicated_key() {
    let native = serde_json::json!({ "id": "x" });
    let merged = merge_attributes(
        native.as_object().cloned().unwrap(),
        vec![("theme".into(), "keystore".into())],
    );
    assert_eq!(merged["realm_attributes"]["theme"], "keystore");
}

#[test]
fn empty_attributes_leave_native_unchanged() {
    let native = serde_json::json!({ "id": "x" });
    let merged = merge_attributes(native.as_object().cloned().unwrap(), vec![]);
    assert_eq!(merged, native);
}

#[test]
fn native_is_deterministic() {
    let a = realm_from_native(
        "master",
        serde_json::json!({ "id": "x", "enabled": true, "z": 1, "a": 2 }),
    );
    let b = realm_from_native(
        "master",
        serde_json::json!({ "id": "x", "enabled": true, "z": 1, "a": 2 }),
    );
    assert_eq!(a, b);
}
