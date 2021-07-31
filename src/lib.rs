//! Parse string to `Duration` .
//!
//! The String value unit support for one of:["y","mon","w","d","h","m","s", "ms", "µs", "ns"]
//!
//! - y:Year. Support string value: ["y" | "year" | "Y" | "YEAR" | "Year"]. e.g. 1y
//!
//! - mon:Month.Support string value: ["mon" | "MON" | "Month" | "month" | "MONTH"]. e.g. 1mon
//!
//! - w:Week.Support string value: ["w" | "W" | "Week" | "WEEK" | "week"]. e.g. 1w
//!
//! - d:Day.Support string value: ["d" | "D" | "Day" | "DAY" | "day"]. e.g. 1d
//!
//! - h:Hour.Support string value: ["h" | "H" | "Hour" | "HOUR" | "hour"]. e.g. 1h
//!
//! - m:Minute.Support string value: ["m" | "M" | "Minute" | "MINUTE" | "minute" | "min" | "MIN"]. e.g. 1m
//!
//! - s:Second.Support string value: ["s" | "S" | "Second" | "SECOND" | "second" | "sec" | "SEC"]. e.g. 1s
//!
//! - ms:Millisecond.Support string value: ["ms" | "MS" | "Millisecond" | "MilliSecond" | "MILLISECOND" | "millisecond" | "mSEC" ]. e.g. 1ms
//!
//! - µs:Microsecond.Support string value: ["µs" | "µS" | "µsecond" | "Microsecond" | "MicroSecond" | "MICROSECOND" | "microsecond" | "µSEC"]. e.g. 1µs
//!
//! - ns:Nanosecond.Support string value: ["ns" | "NS" | "Nanosecond" | "NanoSecond" | "NANOSECOND" | "nanosecond" | "nSEC"]. e.g. 1ns
//!
//! Also,`duration_str` support time duration simple evaluation(+,*). See example:
//!
//! # Example
//!
//! ```rust
//!
//! use duration_str::parse;
//! use std::time::Duration;
//!
//! let duration = parse("1d").unwrap();
//! assert_eq!(duration, Duration::new(24 * 60 * 60, 0));
//!
//! let duration = parse("3m+31").unwrap(); //the default duration unit is second.
//! assert_eq!(duration, Duration::new(211, 0));
//!
//! let duration = parse("3m + 31").unwrap(); //the default duration unit is second.
//! assert_eq!(duration, Duration::new(211, 0));
//!
//! let duration = parse("3m + 13s + 29ms").unwrap();
//! assert_eq!(duration, Duration::new(193, 29 * 1000 * 1000 + 0 + 0));
//!
//! let duration = parse("3m + 1s + 29ms +17µs").unwrap();
//! assert_eq!(
//!     duration,
//!     Duration::new(181, 29 * 1000 * 1000 + 17 * 1000 + 0)
//! );
//!
//! let duration = parse("1m*10").unwrap(); //the default duration unit is second.
//! assert_eq!(duration, Duration::new(600, 0));
//!
//! let duration = parse("1m*10ms").unwrap();
//! assert_eq!(duration, Duration::new(0, 600 * 1000 * 1000));
//!
//! let duration = parse("1m * 1ns").unwrap();
//! assert_eq!(duration, Duration::new(0, 60));
//!
//! let duration = parse("1m * 1m").unwrap();
//! assert_eq!(duration, Duration::new(3600, 0));
//! let duration = parse("42µs").unwrap();
//! assert_eq!(duration,Duration::from_micros(42));
//!
//! ```
//!
//! # deserialize to std::time::Duration
//! `deserialize_duration` Use in struct.
//! ```rust
//! use duration_str::deserialize_duration;
//! use serde::*;
//! use std::time::Duration;
//!
//! #[derive(Debug, Deserialize)]
//! struct Config {
//!     #[serde(deserialize_with = "deserialize_duration")]
//!     time_ticker: Duration,
//! }
//!
//! fn needless_main() {
//!     let json = r#"{"time_ticker":"1m+30"}"#;
//!     let config: Config = serde_json::from_str(json).unwrap();
//!     assert_eq!(config.time_ticker, Duration::new(60 + 30, 0));
//!
//!     let json = r#"{"time_ticker":"1m+30s"}"#;
//!     let config: Config = serde_json::from_str(json).unwrap();
//!     assert_eq!(config.time_ticker, Duration::new(60 + 30, 0));
//! }
//!
//! ```
//!
//!
//! # Also use `deserialize_duration_chrono` function with chrono:Duration
//!
//! ```rust
//! use chrono::Duration;
//! use duration_str::deserialize_duration_chrono;
//! use serde::*;
//!
//! #[derive(Debug, Deserialize)]
//! struct Config {
//!     #[serde(deserialize_with = "deserialize_duration_chrono")]
//!     time_ticker: Duration,
//! }
//!
//! fn needless_main() {
//!     let json = r#"{"time_ticker":"1m+30"}"#;
//!     let config: Config = serde_json::from_str(json).unwrap();
//!     assert_eq!(config.time_ticker, Duration::seconds(60 + 30));
//! }
//! ```
//!

use anyhow::anyhow;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::opt;
use nom::error::{ErrorKind, ParseError};
use nom::sequence::tuple;
use nom::AsChar;
use nom::{IResult, InputTakeAtPosition};
use std::time::Duration;

#[cfg(feature = "chrono")]
use chrono::Duration as CDuration;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;

#[derive(Debug, Eq, PartialEq)]
enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
    MilliSecond,
    MicroSecond,
    NanoSecond,
}

const ONE_MICROSECOND_NANOSECOND: u64 = 1000;
const ONE_MILLISECOND_NANOSECOND: u64 = 1000 * ONE_MICROSECOND_NANOSECOND;
const ONE_SECOND_NANOSECOND: u64 = 1000 * ONE_MILLISECOND_NANOSECOND;
const ONE_MINUTE_NANOSECOND: u64 = 60 * ONE_SECOND_NANOSECOND;
const ONE_HOUR_NANOSECOND: u64 = 60 * ONE_MINUTE_NANOSECOND;
const ONE_DAY_NANOSECOND: u64 = 24 * ONE_HOUR_NANOSECOND;
const ONE_WEEK_NANOSECOND: u64 = 7 * ONE_DAY_NANOSECOND;
const ONE_MONTH_NANOSECOND: u64 = 30 * ONE_DAY_NANOSECOND;
const ONE_YEAR_NANOSECOND: u64 = 365 * ONE_DAY_NANOSECOND;

// const ONE_SECOND_DECIMAL: Decimal = 1_000_000_000.into();
fn one_second_decimal() -> Decimal {
    1_000_000_000.into()
}

impl TimeUnit {
    fn duration(&self, time_str: &str) -> anyhow::Result<u64> {
        let time = time_str.parse::<u64>()?;
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
        Ok(time * unit)
    }
}

const PLUS: &str = "+";
const STAR: &str = "*";

#[derive(Debug, Eq, PartialEq, Clone)]
enum CondUnit {
    Plus,
    Star,
}

impl CondUnit {
    fn init() -> (Self, u64) {
        (CondUnit::Star, ONE_SECOND_NANOSECOND)
    }

    fn change_duration(&self) -> u64 {
        match self {
            CondUnit::Plus => 0,
            CondUnit::Star => ONE_SECOND_NANOSECOND,
        }
    }

    fn calc(&self, x: u64, y: u64) -> anyhow::Result<Duration> {
        let nano_second = match self {
            CondUnit::Plus => x + y,
            CondUnit::Star => {
                let x: Decimal = x.into();
                let y: Decimal = y.into();
                let ret =
                    (x / one_second_decimal()) * (y / one_second_decimal()) * one_second_decimal();
                ret.to_u64()
                    .ok_or_else(|| anyhow!("type of Decimal:{} convert to u64 error", ret))?
            }
        };
        Ok(Duration::from_nanos(nano_second))
    }
}

trait Calc<T> {
    fn calc(&self) -> anyhow::Result<T>;
}

impl Calc<(CondUnit, u64)> for Vec<(&str, CondUnit, TimeUnit)> {
    fn calc(&self) -> anyhow::Result<(CondUnit, u64)> {
        let (mut init_cond, mut init_duration) = CondUnit::init();
        for (index, (val, cond, time_unit)) in self.iter().enumerate() {
            if index == 0 {
                init_cond = cond.clone();
                init_duration = init_cond.change_duration();
            } else if &init_cond != cond {
                return Err(anyhow!(
                    "not support '{}' with '{}' calculate",
                    init_cond.to_string(),
                    cond.to_string()
                ));
            }
            match init_cond {
                CondUnit::Plus => init_duration += time_unit.duration(val)?,
                CondUnit::Star => {
                    let time: Decimal = time_unit.duration(val)?.into();
                    let i = time / one_second_decimal();
                    let mut init: Decimal = init_duration.into();
                    init *= i;
                    init_duration = init
                        .to_u64()
                        .ok_or_else(|| anyhow!("type of Decimal:{} convert to u64 error", init))?;
                }
            }
        }
        Ok((init_cond, init_duration))
    }
}

impl ToString for CondUnit {
    fn to_string(&self) -> String {
        match self {
            Self::Plus => PLUS.to_string(),
            Self::Star => STAR.to_string(),
        }
    }
}

fn unit1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Copy,
{
    input.split_at_position1_complete(
        |item| !(item.is_alpha() || item.as_char() == 'µ'),
        ErrorKind::Alpha,
    )
}

fn time_unit(input: &str) -> IResult<&str, TimeUnit> {
    let (input, out) = unit1(input)?;
    match out.to_lowercase().as_str() {
        "y" | "year" => Ok((input, TimeUnit::Year)),
        "mon" | "month" => Ok((input, TimeUnit::Month)),
        "w" | "week" => Ok((input, TimeUnit::Week)),
        "d" | "day" => Ok((input, TimeUnit::Day)),
        "h" | "hour" => Ok((input, TimeUnit::Hour)),
        "m" | "min" | "minute" => Ok((input, TimeUnit::Minute)),
        "s" | "sec" | "second" => Ok((input, TimeUnit::Second)),
        "ms" | "msec" | "millisecond" => Ok((input, TimeUnit::MilliSecond)),
        "µs" | "µsec" | "µsecond" | "microsecond" => Ok((input, TimeUnit::MicroSecond)),
        "ns" | "nsec" | "nanosecond" => Ok((input, TimeUnit::NanoSecond)),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            "expect one of [y,mon,w,d,h,m,s,ms,µs,ns]",
            ErrorKind::Alpha,
        ))),
    }
}

fn cond_unit(input: &str) -> IResult<&str, CondUnit> {
    let (input, out) =
        input.split_at_position1_complete(|item| !matches!(item, '+' | '*'), ErrorKind::Char)?;
    match out {
        PLUS => Ok((input, CondUnit::Plus)),
        STAR => Ok((input, CondUnit::Star)),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            "expect one of [+,*]",
            ErrorKind::Char,
        ))),
    }
}

fn parse_time(input: &str) -> IResult<&str, (&str, TimeUnit)> {
    tuple((digit1, time_unit))(input)
}

fn cond_time(input: &str) -> IResult<&str, Vec<(&str, CondUnit, TimeUnit)>> {
    let mut vec = vec![];
    let mut input = input;
    while !input.trim().is_empty() {
        let (in_input, (_, cond, _, out, opt_unit)) =
            tuple((multispace0, cond_unit, multispace0, digit1, opt(unit1)))(input)?;
        input = in_input;
        if let Some(unit) = opt_unit {
            let (_, time_unit) = time_unit(unit)?;
            vec.push((out, cond, time_unit));
        } else {
            vec.push((out, cond, TimeUnit::Second));
        }
    }
    Ok(("", vec))
}

/// parse string to `std::time::Duration`
pub fn parse(input: &str) -> anyhow::Result<Duration> {
    let (in_input, ((time_str, time_unit), cond_opt)) =
        tuple((parse_time, opt(cond_time)))(input).map_err(|e| anyhow!("parse error:{}", e))?;
    if !in_input.is_empty() && cond_opt.is_none() {
        return Err(anyhow!(
            "not support duration string:[{}],cause by:[{}],",
            input,
            in_input
        ));
    }
    let (init_cond, init_duration) = cond_opt
        .map(|val| val.calc())
        .unwrap_or_else(|| Ok(CondUnit::init()))?;
    let unit_time = time_unit.duration(time_str)?;
    let duration = init_cond.calc(unit_time, init_duration)?;
    Ok(duration)
}

/// convert Into<String> to `std::time::Duration`
///
/// # Example
///
/// ```rust
///
/// use duration_str::parse;
/// use std::time::Duration;
///
/// let duration = parse("1d").unwrap();
/// assert_eq!(duration,Duration::new(24*60*60,0));
///
/// let duration = parse("3m+31").unwrap();
/// assert_eq!(duration,Duration::new(211,0));
///
/// let duration = parse("3m + 31").unwrap();
/// assert_eq!(duration,Duration::new(211,0));
///
/// let duration = parse("1m*10").unwrap();
/// assert_eq!(duration,Duration::new(600,0));
///
/// let duration = parse("1m * 10").unwrap();
/// assert_eq!(duration,Duration::new(600,0));
/// ```
///
pub fn parse_std<S: Into<String>>(input: S) -> anyhow::Result<Duration> {
    let input = input.into();
    parse(input.as_str())
}

/// convert Into<String> to `chrono::Duration`
///
/// # Example
///
/// ```rust
///
/// use duration_str::parse_chrono;
/// use chrono::Duration;
///
/// let duration = parse_chrono("1d").unwrap();
/// assert_eq!(duration,Duration::seconds(24*60*60));
///
/// let duration = parse_chrono("3m+31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// let duration = parse_chrono("3m + 31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// let duration = parse_chrono("1m*10").unwrap();
/// assert_eq!(duration,Duration::seconds(600));
///
/// let duration = parse_chrono("1m * 10").unwrap();
/// assert_eq!(duration,Duration::seconds(600));
/// ```
///
///
#[cfg(feature = "chrono")]
pub fn parse_chrono<S: Into<String>>(input: S) -> anyhow::Result<chrono::Duration> {
    let std_duration = parse_std(input)?;
    let duration = chrono::Duration::from_std(std_duration)?;
    Ok(duration)
}

macro_rules! des_duration {
    ($name:ident,$duration_type:ident,$fn_name:ident,$parse:ident) => {
        struct $name;
        impl<'de> serde::de::Visitor<'de> for $name {
            type Value = $duration_type;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expect duration string,e.g:'1min+30'")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let duration = $parse(s).map_err(serde::de::Error::custom)?;
                Ok(duration)
            }
        }

        pub fn $fn_name<'de, D>(deserializer: D) -> Result<$duration_type, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_any($name)
        }
    };
}

des_duration!(DurationStd, Duration, deserialize_duration, parse_std);

#[cfg(feature = "chrono")]
des_duration!(
    DurationChrono,
    CDuration,
    deserialize_duration_chrono,
    parse_chrono
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_unit() {
        let (input, format) = time_unit("m123").unwrap();
        assert_eq!(input, "123");
        assert_eq!(format, TimeUnit::Minute);
    }

    #[test]
    fn test_parse_time() {
        let (input, (out, format)) = parse_time("123m").unwrap();
        assert_eq!(input, "");
        assert_eq!(out, "123");
        assert_eq!(format, TimeUnit::Minute);
    }

    #[test]
    fn test_cond_unit() {
        let (input, format) = cond_unit("*123").unwrap();
        assert_eq!(input, "123");
        assert_eq!(format, CondUnit::Star);
    }

    #[test]
    fn test_cond_time() {
        let (input, out) = cond_time(" * 60").unwrap();
        assert_eq!(input, "");
        assert_eq!(out, vec![("60", CondUnit::Star, TimeUnit::Second)]);
    }

    #[test]
    fn test_cond_time2() {
        let (input, out) = cond_time(" * 60*30").unwrap();
        assert_eq!(input, "");
        assert_eq!(
            out,
            vec![
                ("60", CondUnit::Star, TimeUnit::Second),
                ("30", CondUnit::Star, TimeUnit::Second)
            ]
        );
    }

    #[test]
    fn test_duration_parse1() {
        let duration = parse("1m+31").unwrap();
        assert_eq!(duration, Duration::new(91, 0))
    }

    #[test]
    fn test_duration_parse2() {
        let duration = parse("1m*60").unwrap();
        assert_eq!(duration, Duration::new(3600, 0))
    }

    #[test]
    fn test_duration_parse3() {
        let duration = parse("1m*60*20").unwrap();
        assert_eq!(duration, Duration::new(72000, 0))
    }

    #[test]
    fn test_duration_parse4() {
        let duration = parse("1m+60+24").unwrap();
        assert_eq!(duration, Duration::new(144, 0))
    }

    #[test]
    fn test_duration_parse5() {
        let duration = parse("1m+60+24 ").unwrap();
        assert_eq!(duration, Duration::new(144, 0))
    }

    #[test]
    fn test_duration_parse6() {
        let duration = parse("0m").unwrap();
        assert_eq!(duration, Duration::new(0, 0))
    }

    #[test]
    fn test_duration_parse7() {
        assert!(parse("0m+3-5").is_err())
    }

    #[test]
    #[cfg(feature = "chrono")]
    fn test_parse_chrono() {
        use chrono::Duration;
        let duration = parse_chrono("1m+60+24 ").unwrap();
        assert_eq!(duration, Duration::seconds(144))
    }

    #[test]
    fn test_deserialize_duration() {
        use serde::*;
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration")]
            time_ticker: Duration,
        }
        let json = r#"{"time_ticker":"1y+30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            Duration::from_nanos(ONE_YEAR_NANOSECOND) + Duration::from_secs(30)
        );
    }

    #[test]
    #[cfg(feature = "chrono")]
    fn test_deserialize_duration_chrono() {
        use chrono::Duration;
        use serde::*;
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration_chrono")]
            time_ticker: Duration,
        }
        let json = r#"{"time_ticker":"1y+30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            Duration::nanoseconds(ONE_YEAR_NANOSECOND as i64) + Duration::seconds(30)
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
}
