// use: cargo bench --features lowercase

use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;
use winnow::ascii::{digit1, multispace0};
use winnow::error::ContextError;
use winnow::token::literal;
use winnow::Parser;

fn parse_duration() {
    let duration = duration_str::parse("2h 37m").unwrap();
    assert_eq!(duration, Duration::new(9420, 0))
}

fn impeccable_duration() {
    let input = "2h 37m";
    let duration = (
        digit1::<_, ContextError>.try_map(str::parse::<u64>),
        literal('h').value(3600),
        multispace0,
        digit1.try_map(str::parse::<u64>),
        literal('m').value(60),
    )
        .map(|(hour, h_unit, _, min, min_unit)| hour * h_unit + min * min_unit)
        .map(|seconds| Duration::new(seconds, 0))
        .parse(input)
        .unwrap();
    assert_eq!(duration, Duration::new(9420, 0))
}

pub fn duration_str_benchmark(c: &mut Criterion) {
    c.bench_function("duration_str", |b| b.iter(parse_duration));
}

pub fn impeccable_benchmark(c: &mut Criterion) {
    c.bench_function("impeccable", |b| b.iter(impeccable_duration));
}

criterion_group!(benches, duration_str_benchmark, impeccable_benchmark);
criterion_main!(benches);
