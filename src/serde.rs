#[cfg(all(feature = "chrono", feature = "serde"))]
use crate::parse_chrono;
use crate::parse_std;
#[cfg(all(feature = "time", feature = "serde"))]
use crate::parse_time;
use std::time::Duration;

#[cfg(all(feature = "chrono", feature = "serde"))]
use chrono::Duration as CDuration;

#[cfg(all(feature = "time", feature = "serde"))]
use time::Duration as TDuration;

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

#[cfg(all(test, feature = "time"))]
mod tests {
    use super::*;
    use crate::ONE_YEAR_NANOSECOND;
    use serde::*;

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_duration_time() {
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration_time")]
            time_ticker: TDuration,
        }
        #[cfg(not(feature = "no_calc"))]
        let json = r#"{"time_ticker":"1y+30"}"#;
        #[cfg(feature = "no_calc")]
        let json = r#"{"time_ticker":"1y30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            TDuration::nanoseconds(ONE_YEAR_NANOSECOND as i64) + TDuration::seconds(30)
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_option_duration_time() {
        use TDuration;

        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_option_duration_time")]
            time_ticker: Option<TDuration>,
        }
        #[cfg(not(feature = "no_calc"))]
        let json = r#"{"time_ticker":"1y+30"}"#;
        #[cfg(feature = "no_calc")]
        let json = r#"{"time_ticker":"1y30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            Some(TDuration::nanoseconds(ONE_YEAR_NANOSECOND as i64) + TDuration::seconds(30))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_unit_with_spaces() {
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration_time")]
            time_ticker: TDuration,
        }
        #[cfg(not(feature = "no_calc"))]
        let json = r#"{"time_ticker":"1 y + 30"}"#;
        #[cfg(feature = "no_calc")]
        let json = r#"{"time_ticker":"1 y  30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            TDuration::nanoseconds(ONE_YEAR_NANOSECOND as i64) + TDuration::seconds(30)
        );
    }

    #[cfg(all(feature = "serde", feature = "chrono"))]
    #[test]
    fn test_deserialize_duration_chrono() {
        use chrono::Duration;
        #[derive(Debug, serde::Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration_chrono")]
            time_ticker: Duration,
        }
        #[cfg(not(feature = "no_calc"))]
        let json = r#"{"time_ticker":"1y+30"}"#;
        #[cfg(feature = "no_calc")]
        let json = r#"{"time_ticker":"1y30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            Duration::nanoseconds(ONE_YEAR_NANOSECOND as i64) + Duration::seconds(30)
        );
    }

    #[cfg(all(feature = "serde", feature = "chrono"))]
    #[test]
    fn test_deserialize_option_duration_chrono() {
        use chrono::Duration;
        #[derive(Debug, serde::Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_option_duration_chrono")]
            time_ticker: Option<Duration>,
        }
        #[cfg(not(feature = "no_calc"))]
        let json = r#"{"time_ticker":"1y+30"}"#;
        #[cfg(feature = "no_calc")]
        let json = r#"{"time_ticker":"1y30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            Some(Duration::nanoseconds(ONE_YEAR_NANOSECOND as i64) + Duration::seconds(30))
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_duration() {
        #[derive(Debug, serde::Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration")]
            time_ticker: std::time::Duration,
        }

        #[cfg(not(feature = "no_calc"))]
        let json = r#"{"time_ticker":"1min+30"}"#;
        #[cfg(feature = "no_calc")]
        let json = r#"{"time_ticker":"1min30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.time_ticker, std::time::Duration::from_secs(90));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_option_duration() {
        #[derive(Debug, serde::Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_option_duration")]
            time_ticker: Option<std::time::Duration>,
        }
        #[cfg(not(feature = "no_calc"))]
        let json = r#"{"time_ticker":"1min+30"}"#;
        #[cfg(feature = "no_calc")]
        let json = r#"{"time_ticker":"1min30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.time_ticker, Some(std::time::Duration::from_secs(90)));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_duration2() {
        #[derive(Debug, serde::Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration")]
            time_ticker: std::time::Duration,
        }
        #[cfg(not(feature = "no_calc"))]
        let json = r#"{"time_ticker":"1y+30"}"#;
        #[cfg(feature = "no_calc")]
        let json = r#"{"time_ticker":"1y30"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config.time_ticker,
            std::time::Duration::from_nanos(ONE_YEAR_NANOSECOND)
                + std::time::Duration::from_secs(30)
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_option_duration2() {
        #[derive(Debug, serde::Deserialize, PartialEq)]
        struct Config {
            #[serde(default, deserialize_with = "deserialize_option_duration")]
            time_ticker: Option<std::time::Duration>,
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
}
