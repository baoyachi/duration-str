use duration_str::parse_std;
use std::time::Duration;

fn main() {
    let duration = parse_std("2h 37m").unwrap();
    assert_eq!(duration, Duration::new(9420, 0));

    let duration = parse_std("2h 37m ").unwrap();
    assert_eq!(duration, Duration::new(9420, 0));

    let duration = parse_std(" 2h 37m").unwrap();
    assert_eq!(duration, Duration::new(9420, 0));

    let duration = parse_std(" 2h 37m    ").unwrap();
    assert_eq!(duration, Duration::new(9420, 0));
}
