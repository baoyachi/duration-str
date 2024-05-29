use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;
use winnow::error::{ErrorKind, FromExternalError, ParserError};
use winnow::stream::Stream;

pub trait RawDebug {
    fn raw(&self) -> String;
}

impl<A> RawDebug for A
where
    A: AsRef<str>,
{
    fn raw(&self) -> String {
        format!("{}", self.as_ref())
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum DError {
    #[error("{0}")]
    ParseError(String),
    #[error("overflow error")]
    OverflowError,
}

const PARTIAL_INPUT_MAX_LEN: usize = 11;

#[derive(Debug, PartialEq, Eq)]
pub struct PError<I> {
    partial_input: I,
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

    pub fn partial_input(&self) -> String
    where
        I: RawDebug,
    {
        let raw = self.partial_input.raw();
        if let Some(offset) = raw
            .char_indices()
            .enumerate()
            .find_map(|(pos, (offset, _))| (PARTIAL_INPUT_MAX_LEN <= pos).then_some(offset))
        {
            format!("{}...", raw.split_at(offset).0)
        } else {
            raw
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_input() {
        let error = PError::new("1234567890abcde", ErrorKind::Complete);
        let partial_input = error.partial_input();
        assert_eq!(partial_input, "1234567890a...");

        let error = PError::new("你好，龙骧虎步龙行龘龘龘", ErrorKind::Complete);
        let partial_input = error.partial_input();
        assert_eq!(partial_input, "你好，龙骧虎步龙行龘龘...");

        let error = PError::new("hello,你好", ErrorKind::Complete);
        let partial_input = error.partial_input();
        assert_eq!(partial_input, "hello,你好");

        let error = PError::new("1mins", ErrorKind::Complete);
        let partial_input = error.partial_input();
        assert_eq!(partial_input, "1mins");

        let error = PError::new("MILLISECONDhah", ErrorKind::Complete);
        let partial_input = error.partial_input();
        assert_eq!(partial_input, "MILLISECOND...");

        let error = PError::new("MILLISECOND", ErrorKind::Complete);
        let partial_input = error.partial_input();
        assert_eq!(partial_input, "MILLISECOND");
    }
}
