use crate::unit::TimeUnit;
use std::time::Duration;

pub trait HumanFormat {
    fn human_format(&self) -> String;
}

const ONE_SECOND_SECOND: u64 = 1;
const ONE_MINUTE_SECOND: u64 = 60 * ONE_SECOND_SECOND;
const ONE_HOUR_SECOND: u64 = 60 * ONE_MINUTE_SECOND;
const ONE_DAY_SECOND: u64 = 24 * ONE_HOUR_SECOND;
const ONE_WEEK_SECOND: u64 = 7 * ONE_DAY_SECOND;
const ONE_MONTH_SECOND: u64 = 30 * ONE_DAY_SECOND;
const ONE_YEAR_SECOND: u64 = 365 * ONE_DAY_SECOND;

fn accrual(val: u64, unit: TimeUnit, format: &mut String) {
    if val > 0 {
        if !format.is_empty() {
            format.push(' ');
        }
        format.push_str(&format!("{}{}", val, unit));
    }
}

fn format_inner(seconds: u64, nanos: u32) -> String {
    if seconds == 0 && nanos == 0 {
        return "0s".to_string();
    }

    let year = seconds / ONE_YEAR_SECOND;
    let ydays = seconds % ONE_YEAR_SECOND;
    let month = ydays / ONE_MONTH_SECOND;
    let mdays = ydays % ONE_MONTH_SECOND;
    let week = mdays / ONE_WEEK_SECOND;
    let wdays = mdays % ONE_WEEK_SECOND;
    let day = wdays / ONE_DAY_SECOND;
    let day_secs = wdays % ONE_DAY_SECOND;
    let hour = day_secs / ONE_HOUR_SECOND;
    let minutes = day_secs % ONE_HOUR_SECOND / ONE_MINUTE_SECOND;
    let second = day_secs % ONE_MINUTE_SECOND;

    let (millis, micros, nano) = (nanos / 1_000_000, nanos / 1000 % 1000, nanos % 1000);

    let mut format = String::new();
    accrual(year, TimeUnit::Year, &mut format);
    accrual(month, TimeUnit::Month, &mut format);
    accrual(week, TimeUnit::Week, &mut format);
    accrual(day, TimeUnit::Day, &mut format);
    accrual(hour, TimeUnit::Hour, &mut format);
    accrual(minutes, TimeUnit::Minute, &mut format);
    accrual(second, TimeUnit::Second, &mut format);
    accrual(millis as u64, TimeUnit::MilliSecond, &mut format);
    accrual(micros as u64, TimeUnit::MicroSecond, &mut format);
    accrual(nano as u64, TimeUnit::NanoSecond, &mut format);

    format
}

impl HumanFormat for Duration {
    fn human_format(&self) -> String {
        let seconds = self.as_secs();
        let nanos = self.subsec_nanos();
        format_inner(seconds, nanos)
    }
}

#[cfg(all(feature = "chrono", feature = "serde"))]
use chrono::Duration as CDuration;

#[cfg(all(feature = "time", feature = "serde"))]
use time::Duration as TDuration;

#[cfg(all(feature = "chrono", feature = "serde"))]
impl HumanFormat for CDuration {
    fn human_format(&self) -> String {
        let seconds = self.num_seconds() as _;
        let nanos = self.subsec_nanos() as _;
        format_inner(seconds, nanos)
    }
}

#[cfg(all(feature = "time", feature = "serde"))]
impl HumanFormat for TDuration {
    fn human_format(&self) -> String {
        let seconds = self.as_seconds_f64() as _;
        let nanos = self.subsec_nanoseconds() as _;
        format_inner(seconds, nanos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn test_human_format() {
        let duration = parse("0s").unwrap();
        assert_eq!(duration.human_format(), "0s");

        let duration = parse("1y 12d 3s").unwrap();
        assert_eq!(duration.human_format(), "1y 1w 5d 3s");

        let duration = parse("751d 1mon 3week 5d 2ns").unwrap();
        assert_eq!(duration.human_format(), "2y 2mon 2w 3d 2ns");

        let duration = parse("    7h    ").unwrap();
        assert_eq!(duration.human_format(), "7h");

        let duration = parse("    7h  1s  ").unwrap();
        assert_eq!(duration.human_format(), "7h 1s");

        let duration = parse("    7h  0s  ").unwrap();
        assert_eq!(duration.human_format(), "7h");
    }

    #[cfg(all(feature = "serde", feature = "chrono"))]
    #[test]
    fn test_human_format_chrono() {
        let duration = crate::parse_chrono("0s").unwrap();
        assert_eq!(duration.human_format(), "0s");

        let duration = crate::parse_chrono("1y 12d 3s").unwrap();
        assert_eq!(duration.human_format(), "1y 1w 5d 3s");

        let duration = crate::parse_chrono("751d 1mon 3week 5d 2ns").unwrap();
        assert_eq!(duration.human_format(), "2y 2mon 2w 3d 2ns");

        let duration = crate::parse_chrono("    7h    ").unwrap();
        assert_eq!(duration.human_format(), "7h");

        let duration = crate::parse_chrono("    7h  1s  ").unwrap();
        assert_eq!(duration.human_format(), "7h 1s");

        let duration = crate::parse_chrono("    7h  0s  ").unwrap();
        assert_eq!(duration.human_format(), "7h");
    }

    #[cfg(all(feature = "serde", feature = "time"))]
    #[test]
    fn test_human_format_time() {
        let duration = crate::parse_time("0s").unwrap();
        assert_eq!(duration.human_format(), "0s");

        let duration = crate::parse_time("1y 12d 3s").unwrap();
        assert_eq!(duration.human_format(), "1y 1w 5d 3s");

        let duration = crate::parse_time("751d 1mon 3week 5d 2ns").unwrap();
        assert_eq!(duration.human_format(), "2y 2mon 2w 3d 2ns");

        let duration = crate::parse_time("    7h    ").unwrap();
        assert_eq!(duration.human_format(), "7h");

        let duration = crate::parse_time("    7h  1s  ").unwrap();
        assert_eq!(duration.human_format(), "7h 1s");

        let duration = crate::parse_time("    7h  0s  ").unwrap();
        assert_eq!(duration.human_format(), "7h");
    }
}
