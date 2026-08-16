//! Integration tests against a live Keycloak + Postgres instance.
//!
//! These validate the two database assumptions that unit/contract tests
//! cannot: the `migration_model` version query and the `row_to_json` realm
//! extraction, against the real Keycloak schema. They also enforce the
//! end-to-end contract — version detection, extraction, completeness
//! verification, and determinism — on a real backend.
//!
//! They are **env-gated**: they run only when `KEYSTATE_TEST_DB_URL` is set
//! (a `postgres://…` URL), so CI without Docker stays green and skips them.
//! With the compose stack up this is
//! `postgres://keycloak:keycloak@localhost:5432/keycloak`:
//!
//! ```sh
//! docker compose up -d
//! KEYSTATE_TEST_DB_URL=postgres://keycloak:keycloak@localhost:5432/keycloak \
//!   cargo test --test integration -- --ignored
//! ```

use keystate_adapter_keycloak::{KeycloakExtractor, SUPPORTED_KEYCLOAK_VERSION, keycloak_manifest};
use keystate_core::port::Extractor;
use keystate_core::testing::{assert_verification_complete, assert_volatile_segregation};
use keystate_core::{ExtractScope, canonical_bytes};

/// The database URL to test against, from the environment.
fn test_db_url() -> Option<String> {
    std::env::var("KEYSTATE_TEST_DB_URL").ok()
}

/// The realm to extract; the Keycloak image creates `master` on first boot.
const TEST_REALM: &str = "master";

#[tokio::test]
#[ignore = "requires a live Keycloak DB via KEYSTATE_TEST_DB_URL"]
async fn detects_backend_and_supported_version() {
    let Some(url) = test_db_url() else {
        eprintln!("skipping: KEYSTATE_TEST_DB_URL not set");
        return;
    };
    let extractor = KeycloakExtractor::new(url).expect("lazy construction");
    let info = extractor.detect().await.expect("detect");
    assert_eq!(info.backend, "keycloak");
    assert_eq!(
        info.detected_version.to_string(),
        SUPPORTED_KEYCLOAK_VERSION
    );
}

#[tokio::test]
#[ignore = "requires a live Keycloak DB via KEYSTATE_TEST_DB_URL"]
async fn extracts_realm_with_common_and_native() {
    let Some(url) = test_db_url() else {
        eprintln!("skipping: KEYSTATE_TEST_DB_URL not set");
        return;
    };
    let extractor = KeycloakExtractor::new(url).expect("lazy construction");
    let scope = ExtractScope::new(TEST_REALM);
    let realm = extractor.extract(&scope).await.expect("extract");

    assert_eq!(realm.realm.name, TEST_REALM);
    assert!(realm.realm.enabled);
    assert_eq!(realm.realm.native["name"], TEST_REALM);
    assert!(
        realm.realm.native.get("id").is_some(),
        "native carries the id"
    );
}

#[tokio::test]
#[ignore = "requires a live Keycloak DB via KEYSTATE_TEST_DB_URL"]
async fn missing_realm_errors() {
    let Some(url) = test_db_url() else {
        eprintln!("skipping: KEYSTATE_TEST_DB_URL not set");
        return;
    };
    let extractor = KeycloakExtractor::new(url).expect("lazy construction");
    let scope = ExtractScope::new("definitely-not-a-realm");
    let err = extractor.extract(&scope).await.expect_err("must fail");
    let text = err.to_string();
    assert!(
        text.contains("not found"),
        "error should mention the realm: {text}"
    );
}

#[tokio::test]
#[ignore = "requires a live Keycloak DB via KEYSTATE_TEST_DB_URL"]
async fn extraction_verifies_complete_and_segregates_volatile() {
    let Some(url) = test_db_url() else {
        eprintln!("skipping: KEYSTATE_TEST_DB_URL not set");
        return;
    };
    let extractor = KeycloakExtractor::new(url).expect("lazy construction");
    let scope = ExtractScope::new(TEST_REALM);
    let realm = extractor.extract(&scope).await.expect("extract");

    let version: keystate_core::version::Version = SUPPORTED_KEYCLOAK_VERSION.parse().unwrap();
    let manifest = keycloak_manifest(&version).expect("manifest for supported version");
    assert_volatile_segregation(&realm, &manifest);
    assert_verification_complete(&realm, &manifest);
}

#[tokio::test]
#[ignore = "requires a live Keycloak DB via KEYSTATE_TEST_DB_URL"]
async fn extraction_is_deterministic() {
    let Some(url) = test_db_url() else {
        eprintln!("skipping: KEYSTATE_TEST_DB_URL not set");
        return;
    };
    let extractor = KeycloakExtractor::new(url).expect("lazy construction");
    let scope = ExtractScope::new(TEST_REALM);
    let a = extractor.extract(&scope).await.expect("extract #1");
    let b = extractor.extract(&scope).await.expect("extract #2");
    let bytes_a = canonical_bytes(&a).expect("canonical #1");
    let bytes_b = canonical_bytes(&b).expect("canonical #2");
    assert_eq!(bytes_a, bytes_b, "two extractions must be byte-identical");
}
