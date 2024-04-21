use crate::{Calc, CondUnit, DError, DResult, TimeUnit};
use std::time::Duration;
use winnow::ascii::{digit1, multispace0};
use winnow::combinator::{alt, opt};
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;

fn unit_abbr(input: &mut &str) -> PResult<TimeUnit> {
    take_while(1.., |c: char| c.is_alpha() || c == 'µ')
        .try_map(str::parse)
        .parse_next(input)
}

fn cond_unit(input: &mut &str) -> PResult<CondUnit> {
    alt(('+'.value(CondUnit::Plus), '*'.value(CondUnit::Star))).parse_next(input)
}

pub(crate) fn parse_expr_time(input: &mut &str) -> PResult<u64> {
    (
        multispace0,
        digit1,
        opt(unit_abbr).map(Option::unwrap_or_default),
    )
        .map(|x| (x.1, x.2))
        .try_map(|(v, unit)| unit.duration(v))
        .parse_next(input)
}

pub(crate) fn cond_time<'a>(input: &mut &'a str) -> PResult<Vec<(&'a str, CondUnit, TimeUnit)>> {
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

pub fn parse(input: impl AsRef<str>) -> DResult<Duration> {
    let input = input.as_ref();
    let (unit_time, cond_opt) = (parse_expr_time, opt(cond_time))
        .parse(input)
        .map_err(|e| DError::DSLError(format!("{}", e)))?;

    let (init_cond, init_duration) = cond_opt
        .map(|val| val.calc())
        .unwrap_or_else(|| Ok(CondUnit::init()))?;
    let duration = init_cond.calc(unit_time, init_duration)?;
    Ok(duration)
}

#[cfg(test)]
#[allow(clippy::identity_op)]
mod tests {
    use super::*;
    use crate::DError::DSLError;
    use crate::{CondUnit, DError, TimeUnit};
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
        assert!(parse("0m+3-5").is_err())
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
            Err(DSLError(
                r#"
10000000000000000y+60
^
overflow error"#
                    .trim()
                    .to_string()
            ))
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
        let result = parse("580y*2");
        assert_eq!(result, Err(DError::OverflowError));
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
