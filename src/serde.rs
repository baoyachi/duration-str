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

/// Trait for types that can be deserialized from a duration string.
#[cfg(feature = "serde")]
pub trait DeserializeDuration<'de>: Sized {
    /// Deserialize this type from a duration string.
    fn deserialize_duration<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>;
}

/// Internal macro to implement DeserializeDuration for Duration and Option<Duration>
#[cfg(feature = "serde")]
macro_rules! impl_deserialize_duration {
    ($duration_type:ty, $parse:ident) => {
        impl<'de> DeserializeDuration<'de> for $duration_type {
            fn deserialize_duration<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct DurationVisitor;
                impl<'de> serde::de::Visitor<'de> for DurationVisitor {
                    type Value = $duration_type;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("expect duration string, e.g: '1min+30'")
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        let duration = $parse(s).map_err(serde::de::Error::custom)?;
                        Ok(duration)
                    }
                }

                deserializer.deserialize_any(DurationVisitor)
            }
        }
    };
}

/// Internal macro to implement DeserializeDuration for Option<Duration>
#[cfg(feature = "serde")]
macro_rules! impl_deserialize_option_duration {
    ($duration_type:ty, $parse:ident) => {
        impl<'de> DeserializeDuration<'de> for Option<$duration_type> {
            fn deserialize_duration<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct OptionDurationVisitor;
                impl<'de> serde::de::Visitor<'de> for OptionDurationVisitor {
                    type Value = Option<$duration_type>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("expect duration string, null, or missing field")
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        if s.is_empty() {
                            return Ok(None);
                        }
                        let duration = $parse(s).map_err(serde::de::Error::custom)?;
                        Ok(Some(duration))
                    }

                    fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        use serde::Deserialize;
                        let s: String = String::deserialize(d)?;
                        if s.is_empty() {
                            return Ok(None);
                        }
                        let duration = $parse(s).map_err(serde::de::Error::custom)?;
                        Ok(Some(duration))
                    }

                    fn visit_unit<E>(self) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(None)
                    }

                    fn visit_none<E>(self) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(None)
                    }
                }

                deserializer.deserialize_option(OptionDurationVisitor)
            }
        }
    };
}

/// Deserialize duration string to Duration or Option<Duration>.
///
/// This function works with both required and optional fields:
///
/// ```ignore
/// // For required Duration field
/// #[serde(deserialize_with = "deserialize_duration")]
/// time_ticker: Duration,
///
/// // For optional Duration field
/// #[serde(default, deserialize_with = "deserialize_duration")]
/// time_ticker: Option<Duration>,
/// ```
#[cfg(feature = "serde")]
pub fn deserialize_duration<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: DeserializeDuration<'de>,
{
    T::deserialize_duration(deserializer)
}

// ==================== Implementations for std::time::Duration ====================

#[cfg(feature = "serde")]
impl_deserialize_duration!(Duration, parse_std);

#[cfg(feature = "serde")]
impl_deserialize_option_duration!(Duration, parse_std);

// ==================== Implementations for chrono::Duration ====================

#[cfg(all(feature = "chrono", feature = "serde"))]
impl_deserialize_duration!(CDuration, parse_chrono);

#[cfg(all(feature = "chrono", feature = "serde"))]
impl_deserialize_option_duration!(CDuration, parse_chrono);

// ==================== Implementations for time::Duration ====================

#[cfg(all(feature = "time", feature = "serde"))]
impl_deserialize_duration!(TDuration, parse_time);

#[cfg(all(feature = "time", feature = "serde"))]
impl_deserialize_option_duration!(TDuration, parse_time);

// ==================== Type-specific functions for convenience ====================

/// Deserialize duration string to `chrono::Duration`.
///
/// This is a convenience function equivalent to `deserialize_duration`.
#[cfg(all(feature = "chrono", feature = "serde"))]
pub fn deserialize_duration_chrono<'de, D>(deserializer: D) -> Result<CDuration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    DeserializeDuration::deserialize_duration(deserializer)
}

/// Deserialize duration string to `time::Duration`.
///
/// This is a convenience function equivalent to `deserialize_duration`.
#[cfg(all(feature = "time", feature = "serde"))]
pub fn deserialize_duration_time<'de, D>(deserializer: D) -> Result<TDuration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    DeserializeDuration::deserialize_duration(deserializer)
}

// ==================== Backward compatible aliases ====================

/// Deprecated: Use `deserialize_duration` instead.
#[cfg(feature = "serde")]
#[deprecated(since = "0.19.0", note = "Use `deserialize_duration` instead")]
pub fn deserialize_option_duration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    DeserializeDuration::deserialize_duration(deserializer)
}

/// Deprecated: Use `deserialize_duration_chrono` instead.
#[cfg(all(feature = "chrono", feature = "serde"))]
#[deprecated(since = "0.19.0", note = "Use `deserialize_duration_chrono` instead")]
pub fn deserialize_option_duration_chrono<'de, D>(
    deserializer: D,
) -> Result<Option<CDuration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    DeserializeDuration::deserialize_duration(deserializer)
}

/// Deprecated: Use `deserialize_duration_time` instead.
#[cfg(all(feature = "time", feature = "serde"))]
#[deprecated(since = "0.19.0", note = "Use `deserialize_duration_time` instead")]
pub fn deserialize_option_duration_time<'de, D>(
    deserializer: D,
) -> Result<Option<TDuration>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    DeserializeDuration::deserialize_duration(deserializer)
}

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
            #[serde(deserialize_with = "deserialize_duration")]
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
            #[serde(default, deserialize_with = "deserialize_duration")]
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
    fn test_deserialize_option_duration_time_null() {
        use TDuration;

        #[derive(Debug, Deserialize, PartialEq)]
        struct Config {
            #[serde(default, deserialize_with = "deserialize_duration")]
            time_ticker: Option<TDuration>,
            name: String,
        }

        // Test with null
        let json = r#"{"time_ticker":null,"name":"foo"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.time_ticker, None);

        // Test with missing field
        let json = r#"{"name":"foo"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.time_ticker, None);

        // Test with empty string
        let json = r#"{"time_ticker":"","name":"foo"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.time_ticker, None);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_unit_with_spaces() {
        #[derive(Debug, Deserialize)]
        struct Config {
            #[serde(deserialize_with = "deserialize_duration")]
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
            #[serde(deserialize_with = "deserialize_duration")]
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
            #[serde(default, deserialize_with = "deserialize_duration")]
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
            #[serde(default, deserialize_with = "deserialize_duration")]
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
            #[serde(default, deserialize_with = "deserialize_duration")]
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

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_option_duration_empty_string() {
        #[derive(Debug, serde::Deserialize, PartialEq)]
        struct Config {
            #[serde(default, deserialize_with = "deserialize_duration")]
            time_ticker: Option<std::time::Duration>,
            name: String,
        }
        // Empty string should be treated as None
        let json = r#"{"time_ticker":"","name":"foo"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config,
            Config {
                time_ticker: None,
                name: "foo".into(),
            }
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_deserialize_option_duration_with_null_using_struct_flatten() {
        #[derive(Debug, serde::Deserialize, PartialEq)]
        struct ConfigSubStruct {
            #[serde(default, deserialize_with = "deserialize_duration")]
            time_ticker: Option<std::time::Duration>,
            name: String,
        }
        // Empty string should be treated as None
        #[derive(Debug, serde::Deserialize, PartialEq)]
        struct Config {
            #[serde(default, flatten)]
            config: ConfigSubStruct,
        }
        // Empty string should be treated as None
        let json = r#"{"time_ticker":null,"name":"foo"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(
            config,
            Config {
                config: ConfigSubStruct {
                    time_ticker: None,
                    name: "foo".into(),
                }
            }
        );
    }

    // Test backward compatibility with deprecated functions
    #[cfg(feature = "serde")]
    #[test]
    #[allow(deprecated)]
    fn test_deprecated_deserialize_option_duration() {
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
}
