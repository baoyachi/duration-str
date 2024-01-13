use crate::TimeUnit;
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::PResult;
use winnow::Parser;

fn time_unit_abbr(input: &mut &str) -> PResult<TimeUnit> {
    take_while(1.., |c: char| c.is_alpha() || c == 'µ')
        .try_map(str::parse)
        .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use winnow::Partial;

    #[test]
    fn test_time_unit_abbr() {
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("y")),
            Ok(("", TimeUnit::Year))
        );
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("mon")),
            Ok(("", TimeUnit::Month))
        );
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("w")),
            Ok(("", TimeUnit::Week))
        );
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("d")),
            Ok(("", TimeUnit::Day))
        );
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("h")),
            Ok(("", TimeUnit::Hour))
        );
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("m")),
            Ok(("", TimeUnit::Minute))
        );
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("s")),
            Ok(("", TimeUnit::Second))
        );
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("ms")),
            Ok(("", TimeUnit::MilliSecond))
        );
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("µs")),
            Ok(("", TimeUnit::MicroSecond))
        );
        assert_eq!(
            time_unit_abbr.parse_peek(&Partial::new("ns")),
            Ok(("", TimeUnit::NanoSecond))
        );
    }
}
