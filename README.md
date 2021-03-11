# duration-str parser

## example
```toml
[dependencies]
duration-str = "0.1" 
```

```rust
fn main() {
    let duration = duration_str::parse("1h*60*60").unwrap();
    println!("{:?}", duration);//3721s
}
```