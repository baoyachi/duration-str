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
//! - µs:Microsecond.Support string value: ["µs" | "µS" | "µsecond" | "us" | "uS" | "usecond" | "Microsecond" | "MicroSecond" | "MICROSECOND" | "microsecond" | "µSEC"]. e.g. 1µs
//!
//! - ns:Nanosecond.Support string value: ["ns" | "NS" | "Nanosecond" | "NanoSecond" | "NANOSECOND" | "nanosecond" | "nSEC"]. e.g. 1ns
//!
//! Also, `duration_str` support time duration simple evaluation(+,*). See examples below.
//!
//! # Example
//! ```rust
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
//! let duration = parse("3m 1s 29ms 17µs").unwrap();
//! assert_eq!(
//!     duration,
//!     Duration::new(181, 29 * 1000 * 1000 + 17 * 1000 + 0)
//! );
//!
//! let duration = parse("3m1s29ms17us").unwrap();
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
//! ```
//!
//! # deserialize to std::time::Duration
//!
#![cfg_attr(not(feature = "serde"), doc = "This requires the `serde` feature")]
//!
#![cfg_attr(not(feature = "serde"), doc = "```ignore")]
#![cfg_attr(feature = "serde", doc = "```rust")]
//! use duration_str::deserialize_duration;
//! use serde::*;
//! use std::time::Duration;
//!
//! /// Uses `deserialize_duration`.
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
//!
//!     let json = r#"{"time_ticker":"3m 1s 29ms 17µs"}"#;
//!     let config: Config = serde_json::from_str(json).unwrap();
//!     assert_eq!(
//!         config.time_ticker,
//!         Duration::new(181, 29 * 1000 * 1000 + 17 * 1000 + 0)
//!     );
//!
//!     let json = r#"{"time_ticker":"3m1s29ms17us"}"#;
//!     let config: Config = serde_json::from_str(json).unwrap();
//!     assert_eq!(
//!         config.time_ticker,
//!         Duration::new(181, 29 * 1000 * 1000 + 17 * 1000 + 0)
//!     );
//! }
//! ```
//!
//! # deserialize to chrono::Duration
#![cfg_attr(
    not(all(feature = "chrono", feature = "serde")),
    doc = "This requires both the `chrono` and `serde` features"
)]
//!
#![cfg_attr(not(all(feature = "chrono", feature = "serde")), doc = "```ignore")]
#![cfg_attr(all(feature = "chrono", feature = "serde"), doc = "```rust")]
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
//!
//!     let json = r#"{"time_ticker":"1m+30s"}"#;
//!     let config: Config = serde_json::from_str(json).unwrap();
//!     assert_eq!(config.time_ticker, Duration::seconds(60 + 30));
//!
//!     let json = r#"{"time_ticker":"3m 1s 29ms 17µs"}"#;
//!     let config: Config = serde_json::from_str(json).unwrap();
//!     assert_eq!(
//!         config.time_ticker,
//!         Duration::minutes(3)
//!             + Duration::seconds(1)
//!             + Duration::milliseconds(29)
//!             + Duration::microseconds(17)
//!     );
//!
//!     let json = r#"{"time_ticker":"3m1s29ms17us"}"#;
//!     let config: Config = serde_json::from_str(json).unwrap();
//!     assert_eq!(
//!         config.time_ticker,
//!         Duration::minutes(3)
//!             + Duration::seconds(1)
//!             + Duration::milliseconds(29)
//!             + Duration::microseconds(17)
//!     );
//! }
//! ```

#[cfg(all(feature = "chrono", feature = "serde"))]
use chrono::Duration as CDuration;

use dls_parser::*;
use nom::combinator::opt;
use nom::sequence::tuple;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::convert::TryFrom;
use std::time::Duration;
use thiserror::Error;
#[cfg(all(feature = "time", feature = "serde"))]
use time::Duration as TDuration;

#[cfg(feature = "chrono")]
pub use naive_date::{
    after_naive_date, after_naive_date_time, before_naive_date, before_naive_date_time,
};

pub type DResult<T> = Result<T, DError>;

#[derive(Error, Debug)]
pub enum DError {
    #[error("dls express error: `{0}`")]
    DSLError(String),
    #[error("parser error: `{0}`")]
    ParseError(String),
    #[error("`{0}`")]
    NormalError(String),
}

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
    fn duration(&self, time_str: &str) -> DResult<u64> {
        let time = time_str
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

    fn calc(&self, x: u64, y: u64) -> DResult<Duration> {
        let nano_second = match self {
            CondUnit::Plus => x + y,
            CondUnit::Star => {
                let x: Decimal = x.into();
                let y: Decimal = y.into();
                let ret =
                    (x / one_second_decimal()) * (y / one_second_decimal()) * one_second_decimal();
                ret.to_u64().ok_or_else(|| {
                    DError::ParseError(format!("type of Decimal:{} convert to u64 error", ret))
                })?
            }
        };
        Ok(Duration::from_nanos(nano_second))
    }
}

trait Calc<T> {
    fn calc(&self) -> DResult<T>;
}

impl Calc<(CondUnit, u64)> for Vec<(&str, CondUnit, TimeUnit)> {
    fn calc(&self) -> DResult<(CondUnit, u64)> {
        let (mut init_cond, mut init_duration) = CondUnit::init();
        for (index, (val, cond, time_unit)) in self.iter().enumerate() {
            if index == 0 {
                init_cond = cond.clone();
                init_duration = init_cond.change_duration();
            } else if &init_cond != cond {
                return Err(DError::NormalError(format!(
                    "not support '{}' with '{}' calculate",
                    init_cond.to_string(),
                    cond.to_string()
                )));
            }
            match init_cond {
                CondUnit::Plus => init_duration += time_unit.duration(val)?,
                CondUnit::Star => {
                    let time: Decimal = time_unit.duration(val)?.into();
                    let i = time / one_second_decimal();
                    let mut init: Decimal = init_duration.into();
                    init *= i;
                    init_duration = init.to_u64().ok_or_else(|| {
                        DError::ParseError(format!("type of Decimal:{} convert to u64 error", init))
                    })?;
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

mod dls_parser {
    use crate::{CondUnit, TimeUnit, PLUS, STAR};
    use nom::{
        character::complete::{digit1, multispace0},
        combinator::opt,
        error::{ErrorKind, ParseError},
        sequence::tuple,
        AsChar, IResult, InputTakeAtPosition,
    };

    pub(crate) fn unit1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
    where
        T: InputTakeAtPosition,
        <T as InputTakeAtPosition>::Item: AsChar + Copy,
    {
        input.split_at_position1_complete(
            |item| !(item.is_alpha() || item.as_char() == 'µ'),
            ErrorKind::Alpha,
        )
    }

    pub(crate) fn time_unit(input: &str) -> IResult<&str, TimeUnit> {
        let (input, out) = unit1(input)?;
        match out.to_lowercase().as_str() {
            "y" | "year" => Ok((input, TimeUnit::Year)),
            "mon" | "month" => Ok((input, TimeUnit::Month)),
            "w" | "week" => Ok((input, TimeUnit::Week)),
            "d" | "day" => Ok((input, TimeUnit::Day)),
            "h" | "hour" | "hr" => Ok((input, TimeUnit::Hour)),
            "m" | "min" | "minute" => Ok((input, TimeUnit::Minute)),
            "s" | "sec" | "second" => Ok((input, TimeUnit::Second)),
            "ms" | "msec" | "millisecond" => Ok((input, TimeUnit::MilliSecond)),
            "µs" | "µsec" | "µsecond" | "us" | "usec" | "usecond" | "microsecond" => {
                Ok((input, TimeUnit::MicroSecond))
            }
            "ns" | "nsec" | "nanosecond" => Ok((input, TimeUnit::NanoSecond)),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                "expect one of [y,mon,w,d,h,m,s,ms,µs,us,ns] or their longer forms",
                ErrorKind::Alpha,
            ))),
        }
    }

    pub(crate) fn cond_unit(input: &str) -> IResult<&str, CondUnit> {
        let (input, out) = input
            .split_at_position1_complete(|item| !matches!(item, '+' | '*'), ErrorKind::Char)?;
        match out {
            PLUS => Ok((input, CondUnit::Plus)),
            STAR => Ok((input, CondUnit::Star)),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                "expect one of [+,*]",
                ErrorKind::Char,
            ))),
        }
    }

    pub(crate) fn parse_expr_time(input: &str) -> IResult<&str, (&str, TimeUnit)> {
        tuple((digit1, time_unit))(input)
    }

    pub(crate) fn cond_time(input: &str) -> IResult<&str, Vec<(&str, CondUnit, TimeUnit)>> {
        let mut vec = vec![];
        let mut input = input;
        while !input.trim().is_empty() {
            let (in_input, (_, opt_cond, _, out, opt_unit)) =
                tuple((multispace0, opt(cond_unit), multispace0, digit1, opt(unit1)))(input)?;
            input = in_input;
            // Add by default.
            let cond = opt_cond.unwrap_or(CondUnit::Plus);
            // Parse unit, default is seconds.
            let time_unit = opt_unit.map_or_else(
                || Ok(TimeUnit::Second),
                |unit| time_unit(unit).map(|(_, time_unit)| time_unit),
            )?;
            vec.push((out, cond, time_unit));
        }
        Ok(("", vec))
    }
}

/// parse string to `std::time::Duration`
pub fn parse(input: &str) -> DResult<Duration> {
    let (in_input, ((time_str, time_unit), cond_opt)) =
        tuple((parse_expr_time, opt(cond_time)))(input)
            .map_err(|e| DError::DSLError(format!("{}", e)))?;
    if !in_input.is_empty() && cond_opt.is_none() {
        return Err(DError::DSLError(format!(
            "unsupported duration string: [{}], caused by: [{}],",
            input, in_input
        )));
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
/// use duration_str::parse;
/// use std::time::Duration;
///
/// // supports units
/// let duration = parse("1d").unwrap();
/// assert_eq!(duration,Duration::new(24*60*60,0));
///
/// // supports addition
/// let duration = parse("3m+31").unwrap();
/// assert_eq!(duration,Duration::new(211,0));
///
/// // spaces are optional
/// let duration = parse("3m + 31").unwrap();
/// assert_eq!(duration,Duration::new(211,0));
///
/// // plus sign is optional
/// let duration = parse("3m  31").unwrap();
/// assert_eq!(duration,Duration::new(211,0));
///
/// // both plus and spaces are optional
/// let duration = parse("3m31").unwrap();
/// assert_eq!(duration,Duration::new(211,0));
///
/// // supports multiplication
/// let duration = parse("1m*10").unwrap();
/// assert_eq!(duration,Duration::new(600,0));
///
/// // spaces are optional
/// let duration = parse("1m * 10").unwrap();
/// assert_eq!(duration,Duration::new(600,0));
/// ```
pub fn parse_std<S: Into<String>>(input: S) -> DResult<Duration> {
    let input = input.into();
    parse(input.as_str())
}

/// convert Into<String> to `chrono::Duration`
///
/// # Example
///
/// ```rust
/// use duration_str::parse_chrono;
/// use chrono::Duration;
///
/// // supports units
/// let duration = parse_chrono("1d").unwrap();
/// assert_eq!(duration,Duration::seconds(24*60*60));
///
/// // supports addition
/// let duration = parse_chrono("3m+31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// // spaces are optional
/// let duration = parse_chrono("3m + 31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// // plus sign is optional
/// let duration = parse_chrono("3m  31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// // both plus and spaces are optional
/// let duration = parse_chrono("3m31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// // supports multiplication
/// let duration = parse_chrono("1m*10").unwrap();
/// assert_eq!(duration,Duration::seconds(600));
///
/// // spaces are optional
/// let duration = parse_chrono("1m * 10").unwrap();
/// assert_eq!(duration,Duration::seconds(600));
/// ```
#[cfg(feature = "chrono")]
pub fn parse_chrono<S: Into<String>>(input: S) -> DResult<chrono::Duration> {
    let std_duration = parse_std(input)?;
    let duration = chrono::Duration::from_std(std_duration)
        .map_err(|e| DError::ParseError(format!("{}", e)))?;
    Ok(duration)
}

/// convert Into<String> to `time::Duration`
///
/// # Example
///
/// ```rust
/// use duration_str::parse_time;
/// use time::Duration;
///
/// // supports units
/// let duration = parse_time("1d").unwrap();
/// assert_eq!(duration,Duration::seconds(24*60*60));
///
/// // supports addition
/// let duration = parse_time("3m+31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// // spaces are optional
/// let duration = parse_time("3m + 31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// // plus sign is optional
/// let duration = parse_time("3m  31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// // both plus and spaces are optional
/// let duration = parse_time("3m31").unwrap();
/// assert_eq!(duration,Duration::seconds(211));
///
/// // supports multiplication
/// let duration = parse_time("1m*10").unwrap();
/// assert_eq!(duration,Duration::seconds(600));
///
/// // spaces are optional
/// let duration = parse_time("1m * 10").unwrap();
/// assert_eq!(duration,Duration::seconds(600));
/// ```
#[cfg(feature = "time")]
pub fn parse_time<S: Into<String>>(input: S) -> DResult<time::Duration> {
    let std_duration = parse_std(input)?;
    let duration =
        time::Duration::try_from(std_duration).map_err(|e| DError::ParseError(format!("{}", e)))?;
    Ok(duration)
}

#[cfg(feature = "chrono")]
mod naive_date {
    use crate::{parse_chrono, DResult};
    use chrono::Utc;

    #[allow(dead_code)]
    pub enum TimeHistory {
        Before,
        After,
    }

    #[cfg(feature = "chrono")]
    pub fn calc_naive_date_time<S: Into<String>>(
        input: S,
        history: TimeHistory,
    ) -> DResult<chrono::NaiveDateTime> {
        let duration = parse_chrono(input)?;
        let time = match history {
            TimeHistory::Before => (Utc::now() - duration).naive_utc(),
            TimeHistory::After => (Utc::now() + duration).naive_utc(),
        };
        Ok(time)
    }

    macro_rules! gen_naive_date_func {
        ($date_time:ident,$date:ident,$history:expr) => {
            #[allow(dead_code)]
            #[cfg(feature = "chrono")]
            pub fn $date_time<S: Into<String>>(input: S) -> DResult<chrono::NaiveDateTime> {
                calc_naive_date_time(input, $history)
            }

            #[allow(dead_code)]
            #[cfg(feature = "chrono")]
            pub fn $date<S: Into<String>>(input: S) -> DResult<chrono::NaiveDate> {
                let date: chrono::NaiveDateTime = calc_naive_date_time(input, $history)?;
                Ok(date.date())
            }
        };
    }

    gen_naive_date_func!(
        before_naive_date_time,
        before_naive_date,
        TimeHistory::Before
    );

    gen_naive_date_func!(after_naive_date_time, after_naive_date, TimeHistory::After);
}

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
macro_rules! des_option_duration {
    ($name:ident,$duration_type:ident,$fn_name:ident,$parse:ident) => {
        struct $name;
        impl<'de> serde::de::Visitor<'de> for $name {
            type Value = Option<$duration_type>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expect duration string,e.g:'1min+30'")
            }

            fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::Deserialize;
                let s: Option<String> = Option::deserialize(d)?;
                if let Some(s) = s {
                    let duration = $parse(s).map_err(serde::de::Error::custom)?;
                    return Ok(Some(duration));
                }
                Ok(None)
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(None)
            }
        }

        pub fn $fn_name<'de, D>(deserializer: D) -> Result<Option<$duration_type>, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_option($name)
        }
    };
}

#[cfg(feature = "serde")]
des_duration!(DurationStd, Duration, deserialize_duration, parse_std);

#[cfg(feature = "serde")]
des_option_duration!(
    OptionDurationStd,
    Duration,
    deserialize_option_duration,
    parse_std
);

#[cfg(all(feature = "chrono", feature = "serde"))]
des_duration!(
    DurationChrono,
    CDuration,
    deserialize_duration_chrono,
    parse_chrono
);

#[cfg(all(feature = "chrono", feature = "serde"))]
des_option_duration!(
    OptionDurationChrono,
    CDuration,
    deserialize_option_duration_chrono,
    parse_chrono
);

#[cfg(all(feature = "time", feature = "serde"))]
des_duration!(
    DurationTime,
    TDuration,
    deserialize_duration_time,
    parse_time
);

#[cfg(all(feature = "time", feature = "serde"))]
des_option_duration!(
    OptionDurationTime,
    TDuration,
    deserialize_option_duration_time,
    parse_time
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
    fn test_parse_expr_time() {
        let (input, (out, format)) = parse_expr_time("123m").unwrap();
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
                ("30", CondUnit::Star, TimeUnit::Second),
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

    #[cfg(feature = "serde")]
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

#[cfg(all(test, feature = "chrono"))]
mod chrono_tests {
    use super::*;
    use chrono::{Datelike, Utc};

    #[test]
    fn test_parse_chrono() {
        use chrono::Duration;
        let duration = parse_chrono("1m+60+24 ").unwrap();
        assert_eq!(duration, Duration::seconds(144))
    }

    #[cfg(feature = "serde")]
    #[test]
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

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_option_duration_chrono() {
        use chrono::Duration;
        use serde::*;
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_option_duration_chrono")]
            time_ticker: Option<Duration>,
        }
        let json = r#"{"time_ticker":"1y+30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            Some(Duration::nanoseconds(ONE_YEAR_NANOSECOND as i64) + Duration::seconds(30))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_duration() {
        use serde::*;
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration")]
            time_ticker: Duration,
        }
        let json = r#"{"time_ticker":"1min+30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.time_ticker, Duration::from_secs(90));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_option_duration() {
        use serde::*;
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_option_duration")]
            time_ticker: Option<Duration>,
        }
        let json = r#"{"time_ticker":"1min+30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.time_ticker, Some(Duration::from_secs(90)));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_option_duration2() {
        use serde::*;
        #[derive(Debug, Deserialize, PartialEq)]
        struct Config {
            #[serde(default, deserialize_with = "deserialize_option_duration")]
            time_ticker: Option<Duration>,
            name: String,
        }
        let json = r#"{"time_ticker":null,"name":"foo"}"#;
        let config: Config = serde_json::from_str(json).unwrap();

        assert_eq!(
            config,
            Config {
                time_ticker: None,
                name: "foo".into(),
            }
        );

        let json = r#"{"name":"foo"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config,
            Config {
                time_ticker: None,
                name: "foo".into(),
            }
        );
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
    use super::*;
    use serde::*;
    use time::Duration;

    #[test]
    fn test_parse_time() {
        let duration = parse_time("1m+60+24 ").unwrap();
        assert_eq!(duration, Duration::seconds(144))
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_duration_time() {
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration_time")]
            time_ticker: Duration,
        }
        let json = r#"{"time_ticker":"1y+30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            Duration::nanoseconds(ONE_YEAR_NANOSECOND as i64) + Duration::seconds(30)
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_option_duration_time() {
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_option_duration_time")]
            time_ticker: Option<Duration>,
        }
        let json = r#"{"time_ticker":"1y+30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            Some(Duration::nanoseconds(ONE_YEAR_NANOSECOND as i64) + Duration::seconds(30))
        );
    }
}
