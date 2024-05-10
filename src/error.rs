use std::fmt::{Display, Formatter};
use thiserror::Error;
use winnow::error::{ErrorKind, FromExternalError, ParserError};
use winnow::stream::Stream;

#[derive(Error, Debug, PartialEq)]
pub enum DError {
    #[error("{0}")]
    ParseError(String),
    #[error("overflow error")]
    OverflowError,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PError<I> {
    pub partial_input: I,
    kind: ErrorKind,
    cause: String,
}

impl<I> PError<I> {
    fn new(input: I, kind: ErrorKind) -> Self {
        PError {
            partial_input: input,
            kind,
            cause: "".to_string(),
        }
    }

    pub fn append_cause<C: AsRef<str>>(mut self, cause: C) -> Self {
        self.cause = cause.as_ref().to_string();
        self
    }
}

impl<I: Stream + Clone> ParserError<I> for PError<I> {
    fn from_error_kind(input: &I, kind: ErrorKind) -> Self {
        PError::new(input.clone(), kind)
    }

    fn append(self, _: &I, _: &<I as Stream>::Checkpoint, _: ErrorKind) -> Self {
        self
    }
}

impl<I: Clone, E: std::error::Error + Send + Sync + 'static> FromExternalError<I, E> for PError<I> {
    #[inline]
    fn from_external_error(input: &I, kind: ErrorKind, e: E) -> Self {
        let mut err = Self::new(input.clone(), kind);
        {
            err.cause = e.to_string();
        }
        err
    }
}

impl<I> Display for PError<I>
where
    I: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "partial_input:{}", self.partial_input)?;
        if !self.cause.is_empty() {
            write!(f, ", {}", self.cause)?;
        }
        Ok(())
    }
}
