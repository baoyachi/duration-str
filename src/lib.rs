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
//! - h:Hour.Support string value: ["h" | "H" | "hr" | "Hour" | "HOUR" | "hour"]. e.g. 1h
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

mod error;
pub(crate) mod macros;
mod parser;
#[cfg(feature = "serde")]
mod serde;
mod unit;

pub use parser::parse;
#[cfg(feature = "serde")]
pub use serde::*;
use std::fmt::{Debug, Display};

use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::str::FromStr;
use std::time::Duration;

use crate::error::DError;
use crate::unit::TimeUnit;
#[cfg(feature = "chrono")]
pub use naive_date::{
    after_naive_date, after_naive_date_time, before_naive_date, before_naive_date_time,
};

pub type DResult<T> = Result<T, DError>;

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

const PLUS: &str = "+";
const STAR: &str = "*";

trait ExpectErr<const LEN: usize> {
    fn expect_val() -> [&'static str; LEN];
    fn expect_err<S: AsRef<str> + Display>(s: S) -> String;
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum CondUnit {
    Plus,
    Star,
}

impl FromStr for CondUnit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(CondUnit::Plus),
            "*" => Ok(CondUnit::Star),
            _ => Err(Self::expect_err(s)),
        }
    }
}

impl ExpectErr<2> for CondUnit {
    fn expect_val() -> [&'static str; 2] {
        ["+", "*"]
    }

    fn expect_err<S: AsRef<str> + Display>(s: S) -> String {
        format!("expect one of:{:?}, but find:{}", Self::expect_val(), s)
    }
}

impl CondUnit {
    fn init() -> (Self, u64) {
        (CondUnit::Star, ONE_SECOND_NANOSECOND)
    }

    fn contain(c: char) -> bool {
        Self::expect_val().contains(&&*c.to_string())
    }

    fn change_duration(&self) -> u64 {
        match self {
            CondUnit::Plus => 0,
            CondUnit::Star => ONE_SECOND_NANOSECOND,
        }
    }

    fn calc(&self, x: u64, y: u64) -> DResult<Duration> {
        let nano_second = match self {
            CondUnit::Plus => x.checked_add(y).ok_or(DError::OverflowError)?,
            CondUnit::Star => {
                let x: Decimal = x.into();
                let y: Decimal = y.into();
                let ret = (x / one_second_decimal())
                    .checked_mul(y / one_second_decimal())
                    .ok_or(DError::OverflowError)?
                    .checked_mul(one_second_decimal())
                    .ok_or(DError::OverflowError)?;
                ret.to_u64().ok_or(DError::OverflowError)?
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
                return Err(DError::ParseError(format!(
                    "not support '{}' with '{}' calculate",
                    init_cond, cond
                )));
            }
            match init_cond {
                CondUnit::Plus => {
                    init_duration = init_duration
                        .checked_add(time_unit.duration(val)?)
                        .ok_or(DError::OverflowError)?;
                }
                CondUnit::Star => {
                    let time: Decimal = time_unit.duration(val)?.into();
                    let i = time / one_second_decimal();
                    let mut init: Decimal = init_duration.into();
                    init = init.checked_mul(i).ok_or(DError::OverflowError)?;
                    init_duration = init.to_u64().ok_or(DError::OverflowError)?;
                }
            }
        }
        Ok((init_cond, init_duration))
    }
}

impl Display for CondUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Plus => PLUS.to_string(),
            Self::Star => STAR.to_string(),
        };
        write!(f, "{}", str)
    }
}

/// convert `Into<String>` to `std::time::Duration`
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
pub fn parse_std(input: impl AsRef<str>) -> Result<Duration, String> {
    parse(input.as_ref())
}

/// convert `Into<String>` to `chrono::Duration`
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
pub fn parse_chrono(input: impl AsRef<str>) -> Result<chrono::Duration, String> {
    let std_duration = parse_std(input)?;
    let duration = chrono::Duration::from_std(std_duration).map_err(|e| e.to_string())?;
    Ok(duration)
}

/// convert `Into<String>` to `time::Duration`
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
pub fn parse_time(input: impl AsRef<str>) -> Result<time::Duration, String> {
    let std_duration = parse_std(input)?;
    let duration = time::Duration::try_from(std_duration).map_err(|e| e.to_string())?;
    Ok(duration)
}

#[cfg(feature = "chrono")]
mod naive_date {
    use crate::parse_chrono;
    use chrono::Utc;

    #[allow(dead_code)]
    pub enum TimeHistory {
        Before,
        After,
    }

    #[cfg(feature = "chrono")]
    pub fn calc_naive_date_time(
        input: impl AsRef<str>,
        history: TimeHistory,
    ) -> Result<chrono::NaiveDateTime, String> {
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
            pub fn $date_time(input: impl AsRef<str>) -> Result<chrono::NaiveDateTime, String> {
                calc_naive_date_time(input, $history)
            }

            #[allow(dead_code)]
            #[cfg(feature = "chrono")]
            pub fn $date(input: impl AsRef<str>) -> Result<chrono::NaiveDate, String> {
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
