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

const PLUS: &str = "+";
const STAR: &str = "*";

#[derive(Debug, Eq, PartialEq, Clone)]
enum CondUnit {
    Plus,
    Star,
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
// 12 * 60 * 60
// 12 * 60 + 60

pub fn parse(input: &str) -> anyhow::Result<Duration> {
    let (_, ((time, time_unit), cond_opt)) = tuple((parse_time, opt(cond_time)))(input).unwrap();
    let mut default_val = 1;
    if let Some(opt) = cond_opt {
        let mut default_cond = CondUnit::Star;

        for (index, (val, cond)) in opt.iter().enumerate() {
            if index == 0 {
                default_cond = cond.clone();
            } else if &default_cond != cond {
                return Err(anyhow!(
                    "not support '{}' with '{}' calculate",
                    default_cond.to_string(),
                    cond.to_string()
                ));
            }

            match default_cond {
                CondUnit::Plus => default_val *= val.parse::<u64>()?,
                CondUnit::Star => default_val += val.parse::<u64>()?,
            }
        }
    }
    let time = time.parse::<u64>()?;

    let duration_time = match time_unit {
        TimeUnit::Year => time * 365 * 24 * 60 * 60,
        TimeUnit::Month => time * 30 * 24 * 60 * 60,
        TimeUnit::Week => time * 7 * 24 * 60 * 60,
        TimeUnit::Day => time * 24 * 60 * 60,
        TimeUnit::Hour => time * 60 * 60,
        TimeUnit::Minute => time * 60,
        TimeUnit::Second => time,
    };
    let second = duration_time + default_val;
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
    fn test_duration_parse() {
        let duration = parse("1h*60*60").unwrap();
        println!("{:?}", duration);
    }
}
