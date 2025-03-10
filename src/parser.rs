use crate::unit::{opt_unit_abbr, TimeUnit};
use crate::{Calc, CondUnit, ExpectErr};
use std::time::Duration;
use winnow::ascii::{digit1, multispace0};
use winnow::combinator::trace;
use winnow::combinator::{alt, cut_err};
use winnow::combinator::{eof, peek, repeat};
use winnow::error::{StrContext, StrContextValue};
use winnow::ModalResult as WResult;
use winnow::Parser;

pub(crate) fn cond_unit1(input: &mut &str) -> WResult<CondUnit> {
    alt(('+'.value(CondUnit::Plus), '*'.value(CondUnit::Star)))
        .context(StrContext::Expected(StrContextValue::Description(
            CondUnit::get_expect_val(),
        )))
        .parse_next(input)
}

fn opt_cond_unit(input: &mut &str) -> WResult<CondUnit> {
    let result = cond_unit1.parse_next(input);
    if result.is_err() {
        let multispace = multispace0::<_, _>;
        if (multispace, eof).parse_next(input).is_ok() {
            // The input result is empty except for spaces. Give `TimeUnit` default value
            return Ok(CondUnit::Plus);
        }

        return cut_err(peek((
            multispace,
            digit1,
            multispace0,
            opt_unit_abbr,
            multispace,
        )))
        .context(StrContext::Expected(StrContextValue::Description(
            CondUnit::get_expect_val(),
        )))
        .value(CondUnit::Plus)
        .parse_next(input);
    }
    result
}

pub(crate) fn parse_expr_time(input: &mut &str) -> WResult<u64> {
    (multispace0, digit1, multispace0, opt_unit_abbr, multispace0)
        .map(|x| (x.1, x.3))
        .try_map(|(v, unit)| unit.duration(v))
        .parse_next(input)
}

pub(crate) fn cond_time<'a>(input: &mut &'a str) -> WResult<Vec<(&'a str, CondUnit, TimeUnit)>> {
    repeat(
        0..,
        (
            multispace0,
            opt_cond_unit,
            multispace0,
            digit1,
            multispace0,
            // Add by default.
            // Parse unit, default is seconds.
            opt_unit_abbr,
            multispace0,
        )
            .map(|x| (x.3, x.1, x.5)),
    )
    .fold(Vec::new, |mut acc: Vec<_>, item| {
        acc.push(item);
        acc
    })
    .parse_next(input)
}

pub fn parse(input: impl AsRef<str>) -> Result<Duration, String> {
    let input = input.as_ref();
    let (unit_time, cond_val) = (parse_expr_time, trace("cond_time", cond_time))
        .parse(input)
        .map_err(|e| format!("{}", e))?;

    let (init_cond, init_duration) = if cond_val.is_empty() {
        CondUnit::init()
    } else {
        cond_val.calc().map_err(|err| err.to_string())?
    };

    let duration = init_cond
        .calc(unit_time, init_duration)
        .map_err(|err| err.to_string())?;
    Ok(duration)
}

#[cfg(test)]
#[allow(clippy::identity_op)]
mod tests {
    use super::*;
    use crate::{catch_err, unit::TimeUnit, CondUnit};

    #[test]
    fn test_parse_expr_time() {
        let (input, val) = parse_expr_time.parse_peek("123m").unwrap();
        assert_eq!(input, "");
        assert_eq!(val, 7380000000000);
    }

    #[test]
    fn test_cond_unit() {
        let (input, format) = cond_unit1.parse_peek("*123").unwrap();
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

        let duration = parse("1m31").unwrap();
        assert_eq!(duration, Duration::new(91, 0));

        let duration = parse("1m31s").unwrap();
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
            catch_err!(parse("0m+3-5")),
            r#"
0m+3-5
    ^
expected ["y", "mon", "w", "d", "h", "m", "s", "ms", "µs", "us", "ns"]"#
                .trim()
        );

        assert_eq!(
            catch_err!(parse("0mxyz")),
            r#"
0mxyz
 ^
expected ["y", "mon", "w", "d", "h", "m", "s", "ms", "µs", "us", "ns"]"#
                .trim()
        );

        assert_eq!(
            catch_err!(parse("3ms-2ms")),
            r#"
3ms-2ms
   ^
expected ['+', '*']"#
                .trim()
        );
    }

    #[test]
    fn test_parse() {
        let duration = parse("1d").unwrap();
        assert_eq!(duration, Duration::new(24 * 60 * 60, 0));

        let duration = parse("   1d").unwrap();
        assert_eq!(duration, Duration::new(24 * 60 * 60, 0));

        let duration = parse("1d   ").unwrap();
        assert_eq!(duration, Duration::new(24 * 60 * 60, 0));

        let duration = parse("   1d   ").unwrap();
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

        let duration = parse("3m + 31").unwrap();
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("3m  31s").unwrap();
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("3m31s0ns").unwrap();
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("  3m 31s 0ns ").unwrap();
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("1d2h3min4s").unwrap();
        assert_eq!(duration, Duration::new(93784, 0));
    }

    #[test]
    fn test_overflow_plus() {
        assert_eq!(
            catch_err!(parse("10000000000000000y+60")),
            r#"
10000000000000000y+60
^
overflow error"#
                .trim()
                .to_string()
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

    #[test]
    fn test_parse_optional_spaces() {
        let duration = parse("1 d").unwrap();
        assert_eq!(duration, Duration::new(24 * 60 * 60, 0));

        let duration = parse("3 m+31").unwrap(); //the default duration unit is second.
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("3 m + 31").unwrap(); //the default duration unit is second.
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("3 m + 13 s + 29 ms").unwrap();
        assert_eq!(duration, Duration::new(193, 29 * 1000 * 1000 + 0 + 0));

        let duration = parse("3 m + 1 s + 29 ms +17µs").unwrap();
        assert_eq!(
            duration,
            Duration::new(181, 29 * 1000 * 1000 + 17 * 1000 + 0)
        );

        let duration = parse("1 m*10").unwrap(); //the default duration unit is second.
        assert_eq!(duration, Duration::new(600, 0));

        let duration = parse("1 m*10 ms").unwrap();
        assert_eq!(duration, Duration::new(0, 600 * 1000 * 1000));

        let duration = parse("1 m * 1ns").unwrap();
        assert_eq!(duration, Duration::new(0, 60));

        let duration = parse("1 m * 1 m").unwrap();
        assert_eq!(duration, Duration::new(3600, 0));

        let duration = parse("3 m + 31").unwrap();
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("3 m  31 s").unwrap();
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("3 m31 s0 ns").unwrap();
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("  3 m 31 s 0 ns ").unwrap();
        assert_eq!(duration, Duration::new(211, 0));

        let duration = parse("1 d2 h3 min 4s").unwrap();
        assert_eq!(duration, Duration::new(93784, 0));
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
