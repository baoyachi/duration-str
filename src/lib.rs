use nom::{IResult, InputTakeAtPosition};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, alpha1, multispace0};
use nom::sequence::tuple;
use nom::combinator::opt;
use nom::error::ErrorKind;
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

#[derive(Debug, Eq, PartialEq)]
enum CondUnit {
    Plus,
    Star,
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
        _ => Err(nom::Err::Error(nom::error::Error::new("expect one of [y,mon,w,d,h,m,s]", ErrorKind::Alpha)))
    }
}

fn cond_unit(input: &str) -> IResult<&str, CondUnit> {
    let (input, out) = input.split_at_position1_complete(|item| !matches!(item, '+' | '*'), ErrorKind::Char)?;
    match out {
        "+" => Ok((input, CondUnit::Plus)),
        "*" => Ok((input, CondUnit::Star)),
        _ => Err(nom::Err::Error(nom::error::Error::new("expect one of [+,*]", ErrorKind::Char)))
    }
}

fn parse_time(input: &str) -> IResult<&str, (&str, TimeUnit)> {
    tuple((digit1, time_unit))(input)
}

fn cond_time(input: &str) -> IResult<&str, Vec<(&str, CondUnit)>> {
    let mut vec = vec![];
    let mut input = input;
    while !input.trim().is_empty() {
        let (in_input, (_, cond, _, out)) = tuple((
            multispace0,
            cond_unit,
            multispace0,
            digit1,
        ))(input)?;
        input = in_input;
        vec.push((out, cond));
    }
    Ok((input, vec))
}
// 12 * 60 * 60
// 12 * 60 + 60

pub fn duration_parse(input: &str) /*-> Result<Duration, String> */ {
    let (_, ((time, time_unit),cond_opt)) = tuple((parse_time, opt(cond_time)))(input).unwrap();
    if let Some(opt) = cond_opt {
        //TODO not support
    }
    // println!("x:{:?}", x);
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
        assert_eq!(out, vec![
            ("60", CondUnit::Star),
            ("30", CondUnit::Star),
        ]);
    }

    #[test]
    fn test_duration_parse() {
        duration_parse("12d* 60*30");
    }
}


// 12 * 60 * 60
// #[test]
// fn test_duration() {
//     use std::time::Duration;
//     let input = "1d";
//     // YY/MM/DD hh::mm::ss
//     //1y == 1Y == 1YEAR == 1year
//     //1mon == 1MONTH == 1MON = 1mon
//     //1w = 1week == 1WEEK == 1W
//     //1d = 1day == 1D
//     //1h == 1hour = 1H
//     //1min == 1MIN = 1minute = 1m
//     //1s = 1Second == 1SECOND
//     //1s-6s ：range
//     // 1,000 皮秒 = 1纳秒 ns
//     // 1,000,000 皮秒 = 1微秒 μs
//     // 1,000,000,000 皮秒 = 1毫秒 μs
//     // 1,000,000,000,000 皮秒 = 1秒 s
//     //1d*60*60
//     let result = parse_year("12y").unwrap();
//     println!("{:?}", result);
// }