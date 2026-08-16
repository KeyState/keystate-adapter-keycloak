//! Field manifests: what a given Keycloak version must expose.
//!
//! The manifest is the completeness contract for a version. It is generated
//! against [`SUPPORTED_KEYCLOAK_VERSION`](crate::db::version::SUPPORTED_KEYCLOAK_VERSION)
//! and handed to `keystate-core`'s verifier, which checks the extracted
//! realm against it. One module per entity, mirroring the extract layer.

pub mod realm;

/// Build the field manifest for the supported Keycloak version.
///
/// Returns `None` if the version is not one this adapter knows a manifest
/// for — the caller should then refuse to extract (see `ensure_supported`).
pub fn keycloak_manifest(
    version: &keystate_core::version::Version,
) -> Option<keystate_core::verify::FieldManifest> {
    realm::realm_manifest(version)
}
