# parse string to std::time::Duration

[![Chrono GitHub Actions](https://github.com/baoyachi/duration-str-rs/workflows/build/badge.svg)](https://github.com/baoyachi/duration-str-rs/actions?query=workflow%3Abuild)
[![Crates.io](https://img.shields.io/crates/v/duration-str.svg)](https://crates.io/crates/duration-str)
[![Docs.rs](https://docs.rs/duration-str/badge.svg)](https://docs.rs/duration-str)


Parse string to `Duration` . The String value unit support for one of:[y,mon,w,d,h,m,s]
- y:Year. Support string value: ["y" | "year" | "Y" | "YEAR" | "Year"]. e.g. 1y
- mon:Month.Support string value: ["mon" | "MON" | "Month" | "month" | "MONTH"]. e.g. 1mon
- w:Week.Support string value: ["w" | "W" | "Week" | "WEEK" | "week"]. e.g. 1w
- d:Day.Support string value: ["d" | "D" | "Day" | "DAY" | "day"]. e.g. 1d
- h:Hour.Support string value: ["h" | "H" | "Hour" | "HOUR" | "hour"]. e.g. 1h
- m:Minute.Support string value: ["m" | "M" | "Minute" | "MINUTE" | "minute"]. e.g. 1m
- m:Second.Support string value: ["s" | "S" | "Second" | "SECOND" | "second"]. e.g. 1s

Also,`duration_str` support time duration simple calcaute(+,*). See example:


## example
```toml
[dependencies]
duration-str = "0.1" 
```

```rust
use duration_str::parse;
use std::time::Duration;

fn main() {
    let duration = parse("1d").unwrap();
    assert_eq!(duration,Duration::new(24*60*60,0));

    let duration = parse("3m+31").unwrap();
    assert_eq!(duration,Duration::new(211,0));

    let duration = parse("3m + 31").unwrap();
    assert_eq!(duration,Duration::new(211,0));

    let duration = parse("1m*10").unwrap();
    assert_eq!(duration,Duration::new(600,0));

    let duration = parse("1m * 10").unwrap();
    assert_eq!(duration,Duration::new(600,0));
}
```

