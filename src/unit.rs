use crate::error::PError;
use crate::{
    CondUnit, DError, DResult, ExpectErr, ONE_DAY_NANOSECOND, ONE_HOUR_NANOSECOND,
    ONE_MICROSECOND_NANOSECOND, ONE_MILLISECOND_NANOSECOND, ONE_MINUTE_NANOSECOND,
    ONE_MONTH_NANOSECOND, ONE_SECOND_NANOSECOND, ONE_WEEK_NANOSECOND, ONE_YEAR_NANOSECOND,
};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use winnow::ascii::multispace0;
use winnow::combinator::{eof, peek};
use winnow::error::{ErrMode, ErrorKind, FromExternalError};
use winnow::stream::{AsChar, Stream};
use winnow::token::{one_of, take_while};
use winnow::PResult;
use winnow::Parser;

#[derive(Debug, Eq, PartialEq, Default, Clone)]
pub(crate) enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    #[default]
    Second,
    MilliSecond,
    MicroSecond,
    NanoSecond,
}

impl Display for TimeUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeUnit::Year => write!(f, "y"),
            TimeUnit::Month => write!(f, "mon"),
            TimeUnit::Week => write!(f, "w"),
            TimeUnit::Day => write!(f, "d"),
            TimeUnit::Hour => write!(f, "h"),
            TimeUnit::Minute => write!(f, "min"),
            TimeUnit::Second => write!(f, "s"),
            TimeUnit::MilliSecond => write!(f, "ms"),
            TimeUnit::MicroSecond => write!(f, "µs"),
            TimeUnit::NanoSecond => write!(f, "ns"),
        }
    }
}

impl TimeUnit {
    pub(crate) fn duration(&self, time_str: impl AsRef<str>) -> DResult<u64> {
        let time = time_str
            .as_ref()
            .parse::<u64>()
            .map_err(|err| DError::ParseError(err.to_string()))?;
        let unit = match self {
            TimeUnit::Year => ONE_YEAR_NANOSECOND,
            TimeUnit::Month => ONE_MONTH_NANOSECOND,
            TimeUnit::Week => ONE_WEEK_NANOSECOND,
            TimeUnit::Day => ONE_DAY_NANOSECOND,
            TimeUnit::Hour => ONE_HOUR_NANOSECOND,
            TimeUnit::Minute => ONE_MINUTE_NANOSECOND,
            TimeUnit::Second => ONE_SECOND_NANOSECOND,
            TimeUnit::MilliSecond => ONE_MILLISECOND_NANOSECOND,
            TimeUnit::MicroSecond => ONE_MICROSECOND_NANOSECOND,
            TimeUnit::NanoSecond => 1,
        };
        time.checked_mul(unit).ok_or(DError::OverflowError)
    }
}

impl FromStr for TimeUnit {
    type Err = DError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &*s.to_lowercase() {
            "y" | "year" => Ok(TimeUnit::Year),
            "mon" | "month" => Ok(TimeUnit::Month),
            "w" | "week" => Ok(TimeUnit::Week),
            "d" | "day" => Ok(TimeUnit::Day),
            "h" | "hour" | "hr" => Ok(TimeUnit::Hour),
            "m" | "min" | "minute" => Ok(TimeUnit::Minute),
            "s" | "sec" | "second" => Ok(TimeUnit::Second),
            "ms" | "msec" | "millisecond" => Ok(TimeUnit::MilliSecond),
            "µs" | "µsec" | "µsecond" | "us" | "usec" | "usecond" | "microsecond" => {
                Ok(TimeUnit::MicroSecond)
            }
            "ns" | "nsec" | "nanosecond" => Ok(TimeUnit::NanoSecond),
            _ => Err(DError::ParseError(Self::expect_err(s))),
        }
    }
}

impl ExpectErr<11> for TimeUnit {
    fn expect_val() -> [&'static str; 11] {
        ["y", "mon", "w", "d", "h", "m", "s", "ms", "µs", "us", "ns"]
    }

    fn expect_err<S: AsRef<str> + Display>(s: S) -> String {
        format!(
            "expect one of :{:?} or their longer forms, but find:{}",
            Self::expect_val(),
            s,
        )
    }
}

pub(crate) fn unit_abbr1<'a>(input: &mut &'a str) -> PResult<TimeUnit, PError<&'a str>> {
    let checkpoint = input.checkpoint();
    let val = take_while(1.., |c: char| c.is_alpha() || c == 'µ')
        .parse_next(input)
        .map_err(|err: ErrMode<PError<_>>| {
            err.map(|x| {
                let partial_input = x.partial_input();
                x.append_cause(TimeUnit::expect_err(partial_input))
            })
        })?;
    str::parse(val).map_err(|err| {
        input.reset(&checkpoint);
        ErrMode::from_external_error(input, ErrorKind::Fail, err)
    })
}

pub(crate) fn opt_unit_abbr<'a>(input: &mut &'a str) -> PResult<TimeUnit, PError<&'a str>> {
    let multispace = multispace0::<_, PError<_>>;
    let result = unit_abbr1(input);
    if result.is_err() {
        if (multispace, eof).parse_next(input).is_ok() {
            // The input result is empty except for spaces. Give `TimeUnit` default value
            return Ok(TimeUnit::default());
        }

        if peek((multispace, one_of(CondUnit::contain)))
            .parse_next(input)
            .is_ok()
        {
            return Ok(TimeUnit::default());
        } else {
            // this result unwrap is safe.
            return Err(result.err().unwrap().cut());
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catch_err;
    use winnow::{Parser, Partial};

    #[test]
    fn test_time_unit_abbr() {
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("y")),
            Ok(("", TimeUnit::Year))
        );
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("mon")),
            Ok(("", TimeUnit::Month))
        );
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("w")),
            Ok(("", TimeUnit::Week))
        );
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("d")),
            Ok(("", TimeUnit::Day))
        );
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("h")),
            Ok(("", TimeUnit::Hour))
        );
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("m")),
            Ok(("", TimeUnit::Minute))
        );
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("s")),
            Ok(("", TimeUnit::Second))
        );
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("ms")),
            Ok(("", TimeUnit::MilliSecond))
        );
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("µs")),
            Ok(("", TimeUnit::MicroSecond))
        );
        assert_eq!(
            unit_abbr1.parse_peek(&Partial::new("ns")),
            Ok(("", TimeUnit::NanoSecond))
        );
    }

    #[test]
    fn test_time_unit() {
        let (input, format) = unit_abbr1.parse_peek("m123").unwrap();
        assert_eq!(input, "123");
        assert_eq!(format, TimeUnit::Minute);
    }

    #[test]
    fn test_unit_abbr1_err() {
        let expect_err = r#"
nys
^
partial_input:nys, expect one of :["y", "mon", "w", "d", "h", "m", "s", "ms", "µs", "us", "ns"] or their longer forms, but find:nys"#;
        assert_eq!(
            catch_err!(unit_abbr1.parse(&Partial::new("nys"))),
            expect_err.trim_start()
        );

        let expect_err = r#"
^
partial_input:, expect one of :["y", "mon", "w", "d", "h", "m", "s", "ms", "µs", "us", "ns"] or their longer forms, but find:"#;
        assert_eq!(catch_err!(unit_abbr1.parse(&Partial::new(""))), expect_err);
    }

    #[test]
    fn test_opt_unit_abbr() {
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("y")),
            Ok(("", TimeUnit::Year))
        );
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("mon")),
            Ok(("", TimeUnit::Month))
        );
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("w")),
            Ok(("", TimeUnit::Week))
        );
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("d")),
            Ok(("", TimeUnit::Day))
        );
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("h")),
            Ok(("", TimeUnit::Hour))
        );
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("m")),
            Ok(("", TimeUnit::Minute))
        );
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("s")),
            Ok(("", TimeUnit::Second))
        );
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("ms")),
            Ok(("", TimeUnit::MilliSecond))
        );
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("µs")),
            Ok(("", TimeUnit::MicroSecond))
        );
        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("ns")),
            Ok(("", TimeUnit::NanoSecond))
        );

        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("")),
            Ok(("", TimeUnit::Second))
        );

        assert_eq!(
            opt_unit_abbr.parse_peek(&Partial::new("        ")),
            Ok(("", TimeUnit::Second))
        );
    }

    #[test]
    fn test_opt_unit_abbr_err() {
        let expect_err = r#"
nys
^
partial_input:nys, expect one of :["y", "mon", "w", "d", "h", "m", "s", "ms", "µs", "us", "ns"] or their longer forms, but find:nys"#;
        assert_eq!(
            catch_err!(opt_unit_abbr.parse(&Partial::new("nys"))),
            expect_err.trim_start()
        );
    }
}
