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
}
