use duration_str::deserialize_duration;
use serde::*;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(deserialize_with = "deserialize_duration")]
    time_ticker: Duration,
}

fn main() {
    let json = r#"{"time_ticker":"1m+30"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.time_ticker, Duration::new(60 + 30, 0));

    let json = r#"{"time_ticker":"1m+30s"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.time_ticker, Duration::new(60 + 30, 0));

    let json = r#"{"time_ticker":"1ns+30s"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.time_ticker, Duration::new(30, 1));

    let json = r#"{"time_ticker":"1ns+30s+ 1min"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.time_ticker, Duration::new(90, 1));

    let json = r#"{"time_ticker":"1ns+30s+ 1min+1"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.time_ticker, Duration::new(91, 1));

    let json = r#"{"time_ticker":"1ns   +30s+ 1min+4µs"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.time_ticker, Duration::new(90, 4001));
}
