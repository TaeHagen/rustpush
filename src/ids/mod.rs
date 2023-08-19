use std::io;

use openssl::{error::ErrorStack, aes::KeyError};

use crate::bags::BagError;

pub mod user;
pub mod signing;
pub mod identity;

#[derive(Debug)]
pub enum IDSError {
    SSLError(ErrorStack),
    PlistError(plist::Error),
    RequestError(reqwest::Error),
    AuthError(plist::Value),
    BagError(BagError),
    CertError(plist::Dictionary),
    RegisterFailed(u64),
    IoError(io::Error),
    LookupFailed(u64),
    KeyError(KeyError),
    TwoFaError
}

impl From<KeyError> for IDSError {
    fn from(value: KeyError) -> Self {
        IDSError::KeyError(value)
    }
}

impl From<io::Error> for IDSError {
    fn from(value: io::Error) -> Self {
        IDSError::IoError(value)
    }
}

impl From<BagError> for IDSError {
    fn from(value: BagError) -> Self {
        IDSError::BagError(value)
    }
}

impl From<ErrorStack> for IDSError {
    fn from(value: ErrorStack) -> Self {
        IDSError::SSLError(value)
    }
}

impl From<plist::Error> for IDSError {
    fn from(value: plist::Error) -> Self {
        IDSError::PlistError(value)
    }
}

impl From<reqwest::Error> for IDSError {
    fn from(value: reqwest::Error) -> Self {
        IDSError::RequestError(value)
    }
}