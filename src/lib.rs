use anyhow::anyhow;
use nom::character::complete::{alpha1, digit1, multispace0};
use nom::combinator::opt;
use nom::error::ErrorKind;
use nom::sequence::tuple;
use nom::{IResult, InputTakeAtPosition};
use std::time::Duration;

#[derive(Debug, Eq, PartialEq)]
enum TimeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

const ONE_MINUTE_SECOND: u64 = 60;
const ONE_HOUR_SECOND: u64 = ONE_MINUTE_SECOND ^ 2;
const ONE_DAY_SECOND: u64 = 24 * ONE_HOUR_SECOND;
const ONE_WEEK_SECOND: u64 = 7 * ONE_DAY_SECOND;
const ONE_MONTH_SECOND: u64 = 30 * ONE_DAY_SECOND;
const ONE_YEAR_SECOND: u64 = 365 * ONE_DAY_SECOND;

impl TimeUnit {
    fn duration(&self, time: u64) -> u64 {
        let unit = match self {
            TimeUnit::Year => ONE_YEAR_SECOND,
            TimeUnit::Month => ONE_MONTH_SECOND,
            TimeUnit::Week => ONE_WEEK_SECOND,
            TimeUnit::Day => ONE_DAY_SECOND,
            TimeUnit::Hour => ONE_HOUR_SECOND,
            TimeUnit::Minute => ONE_MINUTE_SECOND,
            TimeUnit::Second => 1,
        };
        time * unit
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
    fn calc(&self, x: u64, y: u64) -> u64 {
        match self {
            CondUnit::Plus => x + y,
            CondUnit::Star => x * y,
        }
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

fn time_unit(input: &str) -> IResult<&str, TimeUnit> {
    let (input, out) = alpha1(input)?;
    match out {
        "y" | "year" | "Y" | "YEAR" | "Year" => Ok((input, TimeUnit::Year)),
        "mon" | "MON" | "Month" | "month" | "MONTH" => Ok((input, TimeUnit::Month)),
        "w" | "W" | "Week" | "WEEK" | "week" => Ok((input, TimeUnit::Week)),
        "d" | "D" | "Day" | "DAY" | "day" => Ok((input, TimeUnit::Day)),
        "h" | "H" | "Hour" | "HOUR" | "hour" => Ok((input, TimeUnit::Hour)),
        "m" | "M" | "Minute" | "MINUTE" | "minute" => Ok((input, TimeUnit::Minute)),
        "s" | "S" | "Second" | "SECOND" | "second" => Ok((input, TimeUnit::Second)),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            "expect one of [y,mon,w,d,h,m,s]",
            ErrorKind::Alpha,
        ))),
    }
}

fn cond_unit(input: &str) -> IResult<&str, CondUnit> {
    let (input, out) =
        input.split_at_position1_complete(|item| !matches!(item, '+' | '*'), ErrorKind::Char)?;
    match out {
        "+" => Ok((input, CondUnit::Plus)),
        "*" => Ok((input, CondUnit::Star)),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            "expect one of [+,*]",
            ErrorKind::Char,
        ))),
    }
}

fn parse_time(input: &str) -> IResult<&str, (&str, TimeUnit)> {
    tuple((digit1, time_unit))(input)
}

fn cond_time(input: &str) -> IResult<&str, Vec<(&str, CondUnit)>> {
    let mut vec = vec![];
    let mut input = input;
    while !input.trim().is_empty() {
        let (in_input, (_, cond, _, out)) =
            tuple((multispace0, cond_unit, multispace0, digit1))(input)?;
        input = in_input;
        vec.push((out, cond));
    }
    Ok((input, vec))
}

pub fn parse(input: &str) -> anyhow::Result<Duration> {
    let (in_input, ((time, time_unit), cond_opt)) =
        tuple((parse_time, opt(cond_time)))(input).unwrap();
    let mut default_cond = CondUnit::Star;
    let mut default_val = 0;
    if !in_input.is_empty() && cond_opt.is_none() {
        return Err(anyhow!(
            "not support duration string:[{}],cause by:[{}],",
            input,
            in_input
        ));
    }

    if let Some(opt) = cond_opt {
        for (index, (val, cond)) in opt.iter().enumerate() {
            if index == 0 {
                default_cond = cond.clone();
                if default_cond == CondUnit::Star {
                    default_val = 1;
                }
            } else if &default_cond != cond {
                return Err(anyhow!(
                    "not support '{}' with '{}' calculate",
                    default_cond.to_string(),
                    cond.to_string()
                ));
            }

            match default_cond {
                CondUnit::Plus => default_val += val.parse::<u64>()?,
                CondUnit::Star => default_val *= val.parse::<u64>()?,
            }
        }
    }
    let time = time.parse::<u64>()?;

    let duration_time = time_unit.duration(time);

    let second = default_cond.calc(duration_time, default_val);
    let duration = Duration::new(second, 0);
    Ok(duration)
}

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
        assert_eq!(out, vec![("60", CondUnit::Star)]);
    }

    #[test]
    fn test_cond_time2() {
        let (input, out) = cond_time(" * 60*30").unwrap();
        assert_eq!(input, "");
        assert_eq!(out, vec![("60", CondUnit::Star), ("30", CondUnit::Star),]);
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
        let duration = parse("0m").unwrap();
        assert_eq!(duration, Duration::new(0, 0))
    }

    #[test]
    fn test_duration_parse6() {
        assert!(parse("0m+3-5").is_err())
    }
}
