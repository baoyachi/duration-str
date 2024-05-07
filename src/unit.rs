use crate::{
    DError, DResult, ExpectErr, ONE_DAY_NANOSECOND, ONE_HOUR_NANOSECOND,
    ONE_MICROSECOND_NANOSECOND, ONE_MILLISECOND_NANOSECOND, ONE_MINUTE_NANOSECOND,
    ONE_MONTH_NANOSECOND, ONE_SECOND_NANOSECOND, ONE_WEEK_NANOSECOND, ONE_YEAR_NANOSECOND,
};
use std::fmt::Display;
use std::str::FromStr;

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
