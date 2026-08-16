//! Database-layer errors for the Keycloak adapter.
//!
//! Kept separate from the extraction/manifest layers so those stay pure and
//! easy to unit test. The [`Extractor`](keystate_core::port::Extractor) port
//! returns `keystate_core`'s error type, so this layer converts into it at
//! the trait boundary.

use keystate_core::error::Error as CoreError;

/// The error type used by the database layer.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A failure from the `keystate-core` layer (serialization, version
    /// parsing, manifest validation, …).
    #[error(transparent)]
    Core(#[from] CoreError),

    /// A database (sqlx) failure.
    #[error("database failure: {0}")]
    Database(#[from] sqlx::Error),

    /// A failure establishing or parsing the connection.
    #[error("connection failure: {0}")]
    Connect(String),

    /// The realm requested by the operator does not exist.
    #[error("realm {realm:?} not found in the Keycloak database")]
    RealmNotFound {
        /// The realm name that was requested.
        realm: String,
    },

    /// The detected Keycloak version is not one this adapter supports.
    #[error("unsupported Keycloak version {detected}: supported version is {supported}")]
    UnsupportedVersion {
        /// Version detected in the database.
        detected: String,
        /// The version this adapter was built against.
        supported: String,
    },

    /// A plain-message failure, for cases that do not warrant a typed variant.
    #[error("{0}")]
    Message(String),

    /// Any other failure.
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Convenience alias used across the database layer.
pub type Result<T> = std::result::Result<T, Error>;

impl From<Error> for CoreError {
    fn from(error: Error) -> Self {
        match error {
            Error::Message(message) => CoreError::Message(message),
            other => CoreError::Other(Box::new(other)),
        }
    }
}
