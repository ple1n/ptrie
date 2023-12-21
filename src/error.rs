//! Errors thrown by the library

use std::error::Error;
use std::fmt;

/// Enum of errors returned by this library
#[derive(Debug)]
pub enum TrieError {
    NotFound(String),
}

impl Error for TrieError {}

impl fmt::Display for TrieError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TrieError::NotFound(ref msg) => write!(f, "{}", msg),
        }
    }
}
