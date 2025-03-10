use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DError {
    #[error("{0}")]
    ParseError(String),
    #[error("overflow error")]
    OverflowError,
}
