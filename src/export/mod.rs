//! Keycloak realm-export rendering.
//!
//! Produces the importable realm-export JSON (`RealmRepresentation` shape)
//! that keycloak-config-cli accepts. This is the round-trip contract the
//! project validates against before every release (see `RELEASE.md` §2):
//! the tool extracts from a live Keycloak database, renders this file, and
//! keycloak-config-cli must be able to import it back.

pub mod realm;

pub use realm::realm_export;
