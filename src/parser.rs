use crate::{Calc, CondUnit, TimeUnit};
use std::fmt::{Display, Formatter};
use std::time::Duration;
use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{alt, eof, opt};
use winnow::error::{ErrMode, ErrorKind, FromExternalError, ParserError};
use winnow::stream::{AsChar, Stream};
use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;

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
        write!(f, "partial_input:`{}`,{}", self.partial_input, self.kind)?;
        if !self.cause.is_empty() {
            write!(f, ",{}", self.cause)?;
        }
        Ok(())
    }
}

fn unit_abbr<'a>(input: &mut &'a str) -> PResult<TimeUnit, PError<&'a str>> {
    let checkpoint = input.checkpoint();
    let val = take_while(1.., |c: char| c.is_alpha() || c == 'µ').parse_next(input)?;
    str::parse(val).map_err(|err| {
        input.reset(&checkpoint);
        ErrMode::from_external_error(input, ErrorKind::Fail, err)
    })
}

fn cond_unit<'a>(input: &mut &'a str) -> PResult<CondUnit, PError<&'a str>> {
    alt(('+'.value(CondUnit::Plus), '*'.value(CondUnit::Star))).parse_next(input)
}

pub(crate) fn parse_expr_time<'a>(input: &mut &'a str) -> PResult<u64, PError<&'a str>> {
    (
        multispace0,
        digit1,
        alt(((multispace0, eof).value(TimeUnit::default()), unit_abbr)),
    )
        .map(|x| (x.1, x.2))
        .try_map(|(v, unit)| unit.duration(v))
        .parse_next(input)
}

pub(crate) fn cond_time<'a>(
    input: &mut &'a str,
) -> PResult<Vec<(&'a str, CondUnit, TimeUnit)>, PError<&'a str>> {
    let mut vec = vec![];
    while !input.trim().is_empty() {
        let (cond, out, time_unit) = (
            multispace0,
            opt(cond_unit).map(|x| x.unwrap_or(CondUnit::Plus)),
            multispace0,
            digit1,
            // Add by default.
            // Parse unit, default is seconds.
            opt(unit_abbr).map(Option::unwrap_or_default),
            multispace0,
        )
            .map(|x| (x.1, x.3, x.4))
            .parse_next(input)?;

        vec.push((out, cond, time_unit));
    }
    Ok(vec)
}

pub fn parse(input: impl AsRef<str>) -> Result<Duration, String> {
    let input = input.as_ref();
    let (unit_time, cond_opt) = (parse_expr_time, opt(cond_time))
        .parse(input)
        .map_err(|e| format!("{}", e))?;

    let (init_cond, init_duration) = cond_opt
        .map(|val| val.calc())
        .unwrap_or_else(|| Ok(CondUnit::init()))
        .map_err(|err| err.to_string())?;
    let duration = init_cond
        .calc(unit_time, init_duration)
        .map_err(|err| err.to_string())?;
    Ok(duration)
}

#[cfg(test)]
#[allow(clippy::identity_op)]
mod tests {
    use super::*;
    use crate::{CondUnit, TimeUnit};
    use winnow::Partial;

    #[test]
    fn test_time_unit_abbr() {
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("y")),
            Ok(("", TimeUnit::Year))
        );
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("mon")),
            Ok(("", TimeUnit::Month))
        );
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("w")),
            Ok(("", TimeUnit::Week))
        );
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("d")),
            Ok(("", TimeUnit::Day))
        );
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("h")),
            Ok(("", TimeUnit::Hour))
        );
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("m")),
            Ok(("", TimeUnit::Minute))
        );
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("s")),
            Ok(("", TimeUnit::Second))
        );
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("ms")),
            Ok(("", TimeUnit::MilliSecond))
        );
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("µs")),
            Ok(("", TimeUnit::MicroSecond))
        );
        assert_eq!(
            unit_abbr.parse_peek(&Partial::new("ns")),
            Ok(("", TimeUnit::NanoSecond))
        );
    }

    #[test]
    fn test_time_unit() {
        let (input, format) = unit_abbr.parse_peek("m123").unwrap();
        assert_eq!(input, "123");
        assert_eq!(format, TimeUnit::Minute);
    }

    #[test]
    fn test_parse_expr_time() {
        let (input, val) = parse_expr_time.parse_peek("123m").unwrap();
        assert_eq!(input, "");
        assert_eq!(val, 7380000000000);
    }

    #[test]
    fn test_cond_unit() {
        let (input, format) = cond_unit.parse_peek("*123").unwrap();
        assert_eq!(input, "123");
        assert_eq!(format, CondUnit::Star);
    }

    #[test]
    fn test_cond_time() {
        let (input, out) = cond_time.parse_peek(" * 60").unwrap();
        assert_eq!(input, "");
        assert_eq!(out, vec![("60", CondUnit::Star, TimeUnit::Second)]);
    }

    #[test]
    fn test_cond_time2() {
        let (input, out) = cond_time.parse_peek(" * 60*30").unwrap();
        assert_eq!(input, "");
        assert_eq!(
            out,
            vec![
                ("60", CondUnit::Star, TimeUnit::Second),
                ("30", CondUnit::Star, TimeUnit::Second),
            ]
        );
    }

    #[test]
    fn test_duration_parse0() {
        let duration = parse("0").unwrap();
        assert_eq!(duration, Duration::new(0, 0));

        let duration = parse("0    ").unwrap();
        assert_eq!(duration, Duration::new(0, 0));

        let duration = parse("     0    ").unwrap();
        assert_eq!(duration, Duration::new(0, 0));

        let duration = parse("1").unwrap();
        assert_eq!(duration, Duration::new(1, 0));

        let duration = parse("0m").unwrap();
        assert_eq!(duration, Duration::new(0, 0));

        let duration = parse("1hr").unwrap();
        assert_eq!(duration, Duration::new(3600, 0));

        let duration = parse("1m+31").unwrap();
        assert_eq!(duration, Duration::new(91, 0));

        let duration = parse("1m*60").unwrap();
        assert_eq!(duration, Duration::new(3600, 0));

        let duration = parse("1m*60*20").unwrap();
        assert_eq!(duration, Duration::new(72000, 0));

        let duration = parse("1m+60+24").unwrap();
        assert_eq!(duration, Duration::new(144, 0));

        let duration = parse("1m+60+24 ").unwrap();
        assert_eq!(duration, Duration::new(144, 0));

        let duration = parse("      1m      +  60 +             24 ").unwrap();
        assert_eq!(duration, Duration::new(144, 0))
    }

    #[test]
    fn test_duration_err() {
        assert_eq!(
            parse("0m+3-5").err().unwrap(),
            r#"
0m+3-5
  ^
partial_input:`+3-5`,error Eof"#
                .trim()
        );

        let err = format!("{}", parse("0mxyz").err().unwrap());
        assert_eq!(err, r#"
0mxyz
 ^
partial_input:`mxyz`,error Fail,`expect one of [y,mon,w,d,h,m,s,ms,µs,us,ns] or their longer forms.but find:mxyz`"#.trim());

        //TODO lost cause, need fix
        let err = format!("{}", parse("3ms-2ms").err().unwrap());
        assert_eq!(
            err,
            r#"
3ms-2ms
   ^
partial_input:`-2ms`,error Eof"#
                .trim()
        );
    }

    #[test]
    fn test_parse() {
        let duration = parse("1d").unwrap();
        assert_eq!(duration, Duration::new(24 * 60 * 60, 0));

        let duration = parse("3m+31").unwrap(); //the default duration unit is second.
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("3m + 31").unwrap(); //the default duration unit is second.
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("3m + 13s + 29ms").unwrap();
        assert_eq!(duration, Duration::new(193, 29 * 1000 * 1000 + 0 + 0));

        let duration = parse("3m + 1s + 29ms +17µs").unwrap();
        assert_eq!(
            duration,
            Duration::new(181, 29 * 1000 * 1000 + 17 * 1000 + 0)
        );

        let duration = parse("1m*10").unwrap(); //the default duration unit is second.
        assert_eq!(duration, Duration::new(600, 0));

        let duration = parse("1m*10ms").unwrap();
        assert_eq!(duration, Duration::new(0, 600 * 1000 * 1000));

        let duration = parse("1m * 1ns").unwrap();
        assert_eq!(duration, Duration::new(0, 60));

        let duration = parse("1m * 1m").unwrap();
        assert_eq!(duration, Duration::new(3600, 0));
    }

    #[test]
    fn test_overflow_plus() {
        let result = parse("10000000000000000y+60");
        assert_eq!(
            result,
            Err(r#"
10000000000000000y+60
^
partial_input:`10000000000000000y+60`,error Verify,overflow error"#
                .trim()
                .to_string())
        );
    }

    #[test]
    fn test_max_mul() {
        let duration = parse("580y*1").unwrap();
        assert_eq!(
            duration,
            std::time::Duration::from_millis(18290880000) * 1000
        );
    }

    #[test]
    fn test_overflow_mul() {
        let err = parse("580y*2").err().unwrap();
        assert_eq!(err, "overflow error");
    }
}

#[cfg(all(test, feature = "chrono"))]
mod chrono_tests {
    use crate::{
        after_naive_date, after_naive_date_time, before_naive_date, before_naive_date_time,
        parse_chrono,
    };
    use chrono::{Datelike, Utc};

    #[test]
    fn test_parse_chrono() {
        use chrono::Duration;
        let duration = parse_chrono("1m+60+24 ").unwrap();
        assert_eq!(duration, Duration::seconds(144))
    }

    #[test]
    fn test_after_naive_date_time() {
        let date = Utc::now().naive_utc().date();
        let jd = date.num_days_from_ce() + 180;
        let date = after_naive_date_time("180d").unwrap();
        assert_eq!(date.num_days_from_ce(), jd)
    }

    #[test]
    fn test_after_naive_date() {
        let date = Utc::now().naive_utc().date();
        let jd = date.num_days_from_ce() + 180;
        let date = after_naive_date("180d").unwrap();
        assert_eq!(date.num_days_from_ce(), jd)
    }

    #[test]
    fn test_before_naive_date_time() {
        let date = Utc::now().naive_utc().date();
        let jd = date.num_days_from_ce() - 180;
        let date = before_naive_date_time("180d").unwrap();
        assert_eq!(date.num_days_from_ce(), jd)
    }

    #[test]
    fn test_before_naive_date() {
        let date = Utc::now().naive_utc().date();
        let jd = date.num_days_from_ce() - 180;
        let date = before_naive_date("180d").unwrap();
        assert_eq!(date.num_days_from_ce(), jd)
    }
}

#[cfg(all(test, feature = "time"))]
mod time_tests {
    use crate::parse_time;
    use time::Duration;

    #[test]
    fn test_parse_time() {
        let duration = parse_time("1m+60+24 ").unwrap();
        assert_eq!(duration, Duration::seconds(144))
    }
}
