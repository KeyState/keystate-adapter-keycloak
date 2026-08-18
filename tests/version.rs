//! Version detection and support-gate tests — pure logic, no live database.

use keystate_adapter_keycloak::db::ensure_supported;
use keystate_core::version::Version;

const SUPPORTED: &str = keystate_adapter_keycloak::SUPPORTED_KEYCLOAK_VERSION;

#[test]
fn supported_version_parses() {
    assert_eq!(
        SUPPORTED.parse::<Version>().unwrap(),
        Version::new(26, 5, 5)
    );
}

#[test]
fn exact_supported_version_is_accepted() {
    let supported: Version = SUPPORTED.parse().unwrap();
    ensure_supported(&supported).expect("exact match is supported");
}

#[test]
fn other_versions_are_rejected() {
    assert!(ensure_supported(&Version::new(26, 7, 1)).is_err());
    assert!(ensure_supported(&Version::new(25, 0, 0)).is_err());
    assert!(ensure_supported(&Version::new(27, 0, 0)).is_err());
}

#[test]
fn connection_options_from_url() {
    let opts = keystate_adapter_keycloak::db::connection::connect_options(
        "postgres://user:pass@localhost:5432/keycloak",
    )
    .expect("valid URL");
    assert_eq!(opts.get_host(), "localhost");
    assert_eq!(opts.get_port(), 5432);
    assert_eq!(opts.get_database(), Some("keycloak"));
}

#[test]
fn reject_malformed_url() {
    assert!(keystate_adapter_keycloak::db::connection::connect_options("not a url").is_err());
}
