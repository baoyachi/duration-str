# parse string to std::time::Duration

<p align="center">
  <img
    width="200"
    src="https://raw.githubusercontent.com/baoyachi/duration-str/master/duration-str.png"
    alt="duration-str parser"
  />
</p>

[![Chrono GitHub Actions](https://github.com/baoyachi/duration-str-rs/actions/workflows/check.yml/badge.svg)](https://github.com/baoyachi/duration-str-rs/actions?query=workflow%3Abuild)
[![Crates.io](https://img.shields.io/crates/v/duration-str.svg)](https://crates.io/crates/duration-str)
[![Docs.rs](https://docs.rs/duration-str/badge.svg)](https://docs.rs/duration-str)
[![Coverage Status](https://coveralls.io/repos/github/baoyachi/duration-str/badge.svg?branch=master)](https://coveralls.io/github/baoyachi/duration-str?branch=master)


## Features:

* 🚀 Strong compatibility, accommodating leading or trailing whitespaces in strings.
* 👍️ Offers [Playground](https://baoyachi.github.io/duration-str/) support for online debugging.
* ⭐ Integrated with the [serde](https://docs.rs/serde) library.
* 🎉 Supports parsing of various `Duration` types:
    * https://doc.rust-lang.org/stable/std/time/struct.Duration.html
    * https://docs.rs/chrono/latest/chrono/struct.Duration.html
    * https://docs.rs/time/latest/time/struct.Duration.html
* 🔥 Enables formatting of `Duration` into human-readable formats.
* 🍻 Provides precise error localization for easy troubleshooting.
* ⚡  Compatible with WebAssembly (wasm).
* 🎨 Adapts to the [humantime](https://docs.rs/humantime/latest/humantime) crate, despite its apparent lack of recent
   updates...

## Performance
Suggestion: It is recommended to enable the `lowercase` feature to improve performance:  
* Default: Calls `to_lowercase()` each time, designed for compatibility, suitable for flexible input but with lower performance.  
* `lowercase` feature: Skips the conversion, ideal for lowercase input scenarios, offering better performance."

## Notice ⚠️

The default duration unit is second.Also use below **duration unit**

## Duration Unit List

Parse string to `Duration` . The String duration unit support for one
of:`["y","mon","w","d","h","m","s", "ms", "µs", "ns"]`

| unit | Description | unit list option(one of)                                                                           | example |
|------|-------------|----------------------------------------------------------------------------------------------------|---------|
| y    | Year        | ["y" , "year" , "Y" , "YEAR" , "Year"]                                                             | 1y      |
| mon  | Month       | ["mon" , "MON" , "Month" , "month" , "MONTH"]                                                      | 1mon    |
| w    | Week        | ["w" , "W" , "Week" ,"WEEK" , "week"]                                                              | 1w      |
| d    | Day         | ["d" , "D" , "Day" , "DAY" , "day"]                                                                | 1d      |
| h    | Hour        | ["h" , "hr" , "H" , "Hour" , "HOUR" , "hour"]                                                      | 1h      |
| m    | Minute      | ["m" , "M" , "Minute" , "MINUTE" , "minute" , "min" , "MIN"]                                       | 1m      |
| s    | Second      | ["s" , "S" , "Second" , "SECOND" , "second" , "sec" , "SEC"]                                       | 1s      |
| ms   | Millisecond | ["ms" , "MS" , "Millisecond" , "MilliSecond" , "MILLISECOND" , "millisecond" , "mSEC"]             | 1ms     |
| µs   | Microsecond | ["µs" , "µS" , "µsecond" , "Microsecond" , "MicroSecond" , "MICROSECOND" , "microsecond" , "µSEC"] | 1µs     |
| ns   | Nanosecond  | ["ns" , "NS" , "Nanosecond" , "NanoSecond" , "NANOSECOND" , "nanosecond" , "nSEC"]                 | 1ns     |

Also,`duration_str` support time duration simple evaluation(+,*). See example:

## example

```toml
[dependencies]
duration-str = "{latest version}" 
```

```rust
use duration_str::parse;
use std::time::Duration;

fn main() {
    let duration = parse("1d").unwrap();
    assert_eq!(duration, Duration::new(24 * 60 * 60, 0));

    let duration = parse("3m+31").unwrap(); //the default duration unit is second.
    assert_eq!(duration, Duration::new(211, 0));

    let duration = parse("3m + 31").unwrap(); //the default duration unit is second.
    assert_eq!(duration, Duration::new(211, 0));

    let duration = parse("3m31s").unwrap();
    assert_eq!(duration, Duration::new(211, 0));

    let duration = parse("3m + 13s + 29ms").unwrap();
    assert_eq!(duration, Duration::new(193, 29 * 1000 * 1000 + 0 + 0));

    let duration = parse("3m + 1s + 29ms +17µs").unwrap();
    assert_eq!(
        duration,
        Duration::new(181, 29 * 1000 * 1000 + 17 * 1000 + 0)
    );

    let duration = parse("1m*10").unwrap(); //the default duration unit is second.
    assert_eq!(duration, Duration::new(600, 0));

    let duration = parse("1m*10ms").unwrap();
    assert_eq!(duration, Duration::new(0, 600 * 1000 * 1000));

    let duration = parse("1m * 1ns").unwrap();
    assert_eq!(duration, Duration::new(0, 60));

    let duration = parse("1m * 1m").unwrap();
    assert_eq!(duration, Duration::new(3600, 0));
}
```

## deserialize in struct

deserialize to std::time::Duration

```rust
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
}
```

* option filed deserialize

```rust
use duration_str::deserialize_option_duration;
use serde::*;
use std::time::Duration;

#[derive(Debug, Deserialize, PartialEq)]
struct Config {
    #[serde(default, deserialize_with = "deserialize_option_duration")]
    time_ticker: Option<Duration>,
    name: String,
}

fn main() {
    let json = r#"{"time_ticker":null,"name":"foo"}"#;
    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(
        config,
        Config {
            time_ticker: None,
            name: "foo".into()
        }
    );

    let json = r#"{"name":"foo"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(
        config,
        Config {
            time_ticker: None,
            name: "foo".into()
        }
    );
}
```

Also you can use `deserialize_duration_chrono` or `deserialize_duration_time` function

### E.g:

```rust
use chrono::Duration;
use duration_str::deserialize_duration_chrono;
use serde::*;

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(deserialize_with = "deserialize_duration_chrono")]
    time_ticker: Duration,
}

fn main() {
    let json = r#"{"time_ticker":"1m+30"}"#;
    let config: Config = serde_json::from_str(json).unwrap();
    assert_eq!(config.time_ticker, Duration::seconds(60 + 30));
}
```

