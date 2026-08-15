//! Read-only PostgreSQL connection for Keycloak extraction.
//!
//! The connection is built once from an operator-supplied URL
//! (`KEYSTATE_DB_URL` by convention, set by the CLI) and shared across the
//! extraction. It is opened **read-only** so the adapter cannot accidentally
//! modify a live Keycloak database.

use sqlx::postgres::{PgConnectOptions, PgPool, PgPoolOptions};

use crate::db::error::{Error, Result};

/// A pooled, read-only connection to the Keycloak PostgreSQL database.
#[derive(Debug, Clone)]
pub struct KeycloakConnection {
    pool: PgPool,
}

impl KeycloakConnection {
    /// Connect to the Keycloak database described by `database_url`.
    ///
    /// The session is forced read-only with a 60s statement timeout, so the
    /// adapter can never modify a live Keycloak database and a hung query
    /// cannot hold a connection forever. Reads run at the database's default
    /// isolation level (which, like a single query per entity, is safe for a
    /// read-only extraction).
    pub async fn connect(database_url: &str) -> Result<Self> {
        let options = connect_options(database_url)?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e| Error::Connect(e.to_string()))?;
        Ok(Self { pool })
    }

    /// Build a connection without touching the network.
    ///
    /// The pool connects lazily on first use, which lets an extractor be
    /// constructed synchronously; connection errors surface on the first
    /// query instead.
    pub fn connect_lazy(database_url: &str) -> Result<Self> {
        let options = connect_options(database_url)?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy_with(options);
        Ok(Self { pool })
    }

    /// The underlying pool, used by the extraction layer.
    pub(crate) fn pool(&self) -> &PgPool {
        &self.pool
    }
}

/// Build connection options with Keystate's read-only session defaults.
///
/// Exposed so tests and tools can inspect the exact options an extractor
/// uses without opening a connection.
pub fn connect_options(database_url: &str) -> Result<PgConnectOptions> {
    database_url
        .parse::<PgConnectOptions>()
        .map_err(|e| Error::Connect(e.to_string()))
        .map(|options| {
            options
                .application_name("keystate-adapter-keycloak")
                .options([
                    ("default_transaction_read_only", "on"),
                    ("statement_timeout", "60000"),
                ])
        })
}
