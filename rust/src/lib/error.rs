//! Errors generated from the mini terminal.

use log::error;

/// Result from mini terminal.
pub type OurResult<X> = Result<X, OurError>;

/// Errors from the tool, or its subcomponents.
#[derive(Debug, Clone)]
pub enum OurError {
    String(String),
    FromHexError(hex::FromHexError),
}

impl std::convert::From<hex::FromHexError> for OurError {
    fn from(fhe: hex::FromHexError) -> Self {
        error!("hex-string conversion failure.");
        OurError::FromHexError(fhe)
    }
}
