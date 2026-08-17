//! Realm-export rendering for the `Realm` entity.
//!
//! Maps the backend-faithful realm row (`native`) into Keycloak's realm-export
//! format — the field names keycloak-config-cli expects — while applying the
//! round-trip contract established by the config-cli import validation:
//!
//! - **no `id` fields**: config-cli regenerates every id on import;
//! - **no id-reference fields** (`defaultRole`, flow fields, scope mappings):
//!   Keycloak applies its defaults when they are absent;
//! - **no `keycloakVersion`**: derived from the live instance, not the file;
//! - **nulls dropped**: absent settings are Keycloak defaults, so omitting
//!   them keeps the file as small as config-cli's own guidance recommends.
//!
//! The realm's identity key and enabled state come from the canonical common
//! fields; the remaining settings are the `REALM` table columns Keycloak
//! persists (see `extract::realm::REALM_QUERY`). Entity collections are empty
//! until per-entity extraction lands; the empty shape is itself importable,
//! which is what the empty-realm baseline proves.

use keystate_core::model::CanonicalRealm;
use serde_json::{Map, Value, json};

/// `REALM` table columns mapped to their realm-export (camelCase) names.
///
/// Every column whose value belongs in the export file appears here; columns
/// that reference other entities by id (`browser_flow`, `default_role`, …)
/// or that have no realm-export equivalent (`master_admin_client`, `social`,
/// …) are deliberately omitted.
const REALM_SETTINGS: &[(&str, &str)] = &[
    ("access_code_lifespan", "accessCodeLifespan"),
    ("user_action_lifespan", "accessCodeLifespanUserAction"),
    ("access_token_lifespan", "accessTokenLifespan"),
    ("account_theme", "accountTheme"),
    ("admin_theme", "adminTheme"),
    ("email_theme", "emailTheme"),
    ("events_enabled", "eventsEnabled"),
    ("events_expiration", "eventsExpiration"),
    ("login_theme", "loginTheme"),
    ("not_before", "notBefore"),
    ("password_policy", "passwordPolicy"),
    ("registration_allowed", "registrationAllowed"),
    ("remember_me", "rememberMe"),
    ("reset_password_allowed", "resetPasswordAllowed"),
    ("ssl_required", "sslRequired"),
    ("sso_idle_timeout", "ssoSessionIdleTimeout"),
    ("sso_max_lifespan", "ssoSessionMaxLifespan"),
    ("verify_email", "verifyEmail"),
    ("login_lifespan", "accessCodeLifespanLogin"),
    (
        "internationalization_enabled",
        "internationalizationEnabled",
    ),
    ("default_locale", "defaultLocale"),
    ("reg_email_as_username", "registrationEmailAsUsername"),
    ("admin_events_enabled", "adminEventsEnabled"),
    ("admin_events_details_enabled", "adminEventsDetailsEnabled"),
    ("edit_username_allowed", "editUsernameAllowed"),
    ("otp_policy_counter", "otpPolicyInitialCounter"),
    ("otp_policy_window", "otpPolicyLookAheadWindow"),
    ("otp_policy_period", "otpPolicyPeriod"),
    ("otp_policy_digits", "otpPolicyDigits"),
    ("otp_policy_alg", "otpPolicyAlgorithm"),
    ("otp_policy_type", "otpPolicyType"),
    ("offline_session_idle_timeout", "offlineSessionIdleTimeout"),
    ("revoke_refresh_token", "revokeRefreshToken"),
    (
        "access_token_life_implicit",
        "accessTokenLifespanForImplicitFlow",
    ),
    ("login_with_email_allowed", "loginWithEmailAllowed"),
    ("duplicate_emails_allowed", "duplicateEmailsAllowed"),
    ("refresh_token_max_reuse", "refreshTokenMaxReuse"),
    ("allow_user_managed_access", "userManagedAccessAllowed"),
    (
        "sso_max_lifespan_remember_me",
        "ssoSessionMaxLifespanRememberMe",
    ),
    (
        "sso_idle_timeout_remember_me",
        "ssoSessionIdleTimeoutRememberMe",
    ),
];

/// Entity collections, empty for the current realm-settings scope. The empty
/// shape must stay importable — that is the baseline keycloak-config-cli
/// validation guarantees.
fn empty_collections() -> Vec<(&'static str, Value)> {
    vec![
        ("clients", json!([])),
        ("clientScopes", json!([])),
        ("groups", json!([])),
        ("identityProviders", json!([])),
        ("identityProviderMappers", json!([])),
        ("authenticationFlows", json!([])),
        ("authenticatorConfig", json!([])),
        ("requiredActions", json!([])),
        ("clientProfiles", json!({ "profiles": [] })),
        ("localizationTexts", json!({})),
        ("browserSecurityHeaders", json!({})),
        ("smtpServer", json!({})),
        ("attributes", json!({})),
        ("components", json!({})),
        ("clientPolicies", json!({ "policies": [] })),
        ("roles", json!({ "realm": [], "client": {} })),
    ]
}

/// Render one canonical realm extraction into the importable realm-export
/// format, per the round-trip contract documented on this module.
///
/// This is a pure function over the canonical model, so it is unit-testable
/// without a database.
pub fn realm_export(canonical: &CanonicalRealm) -> Value {
    let mut out: Map<String, Value> = Map::new();

    if let Some(native) = canonical.realm.native.as_object() {
        for (db_column, export_key) in REALM_SETTINGS {
            if let Some(value) = native.get(*db_column) {
                if !value.is_null() {
                    out.insert((*export_key).to_string(), value.clone());
                }
            }
        }
    }

    out.insert("realm".into(), Value::String(canonical.realm.name.clone()));
    out.insert("enabled".into(), Value::Bool(canonical.realm.enabled));

    for (key, value) in empty_collections() {
        out.insert(key.to_string(), value);
    }

    Value::Object(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use keystate_core::model::{BackendInfo, Realm};
    use keystate_core::version::Version;

    fn canonical(native: Value) -> CanonicalRealm {
        CanonicalRealm {
            backend: BackendInfo {
                backend: "keycloak".into(),
                detected_version: Version::new(26, 5, 5),
                extra: Default::default(),
            },
            realm: Realm {
                name: "master".into(),
                enabled: true,
                native,
            },
            ..CanonicalRealm::default()
        }
    }

    #[test]
    fn maps_db_columns_to_export_field_names() {
        let native = json!({
            "access_code_lifespan": 60,
            "access_token_lifespan": 60,
            "ssl_required": "EXTERNAL",
            "sso_idle_timeout": 1800,
            "otp_policy_alg": "HmacSHA1",
            "enabled": true,
            "name": "master",
        });
        let value = realm_export(&canonical(native));

        assert_eq!(value["realm"], "master");
        assert_eq!(value["enabled"], true);
        assert_eq!(value["accessCodeLifespan"], 60);
        assert_eq!(value["accessTokenLifespan"], 60);
        assert_eq!(value["sslRequired"], "EXTERNAL");
        assert_eq!(value["ssoSessionIdleTimeout"], 1800);
        assert_eq!(value["otpPolicyAlgorithm"], "HmacSHA1");
    }

    #[test]
    fn omits_ids_id_references_and_nulls() {
        let native = json!({
            "id": "5fc63700-7f53-49fc-845b-ebcbf1d61cfe",
            "browser_flow": "fca41d60-b0eb-4fc6-b509-08f6187b78c6",
            "default_role": "1e13c58d-bc5a-4113-a991-32c890fbda2e",
            "master_admin_client": "dcb686d1-500f-4509-9b46-20105e3ae697",
            "account_theme": null,
            "default_locale": null,
        });
        let value = realm_export(&canonical(native));
        let text = value.to_string();

        assert!(!text.contains("5fc63700"), "realm id leaked into export");
        assert!(!text.contains("fca41d60"), "flow id leaked into export");
        assert!(
            !text.contains("1e13c58d"),
            "default role id leaked into export"
        );
        assert!(
            !text.contains("dcb686d1"),
            "master admin client id leaked into export"
        );
        assert!(
            !text.contains("accountTheme"),
            "null theme should be omitted"
        );
        assert!(
            !text.contains("defaultLocale"),
            "null locale should be omitted"
        );
    }

    #[test]
    fn includes_the_importable_empty_collection_shape() {
        let value = realm_export(&canonical(json!({"enabled": true, "name": "master"})));

        assert_eq!(value["clients"], json!([]));
        assert_eq!(value["clientScopes"], json!([]));
        assert_eq!(value["roles"], json!({ "realm": [], "client": {} }));
        assert_eq!(value["clientProfiles"], json!({ "profiles": [] }));
        assert_eq!(value["clientPolicies"], json!({ "policies": [] }));
        assert_eq!(value["authenticationFlows"], json!([]));
        assert_eq!(value["components"], json!({}));
    }

    #[test]
    fn deterministic_across_calls() {
        let value = realm_export(&canonical(json!({"enabled": true, "name": "master"})));
        assert_eq!(
            value,
            realm_export(&canonical(json!({"enabled": true, "name": "master"})))
        );
    }
}
