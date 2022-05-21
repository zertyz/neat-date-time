# neat-date-time crate

(native) Types & operations to allow space efficient storage of dates and times.

# Problem statement

Rust's `std` (as well as the `chrono` library) provides general date & time structures, meaning they offer:
   * general range
   * general precision

A storage price is payed for such generalization. Thus, there is a possible space optimization by fine tuning those two properties for domain specific needs.

# Application example

Lets take the example of stock markets. Trades can be grouped in daily sets and individual trades will happen between the oppening and closing time of the trading session. The possible optimizations are:
   * date range: by using a single u16, we're able to represent ~179 years -- by using an epoch date, any `std` or `chrono` date may be converted to u16 back and forth;
   * time range & precision: we have two options here: use the full 24h range (with as much precision as possible) or use a partial range -- lets say, 12h -- at the double the precision. If we use a u32 for time, a 24h range would allow a precision of ~20.117µs (or, precisely, `1/((2^32)/86400)*1e6`µs). On the other hand, if we want a precision of exactly 10µs, a u32 would be able to represent 11:55:49.67296s (from the formula: `s*1e6 / (2^32) = µs_precision`; resolved to `s=(2^32)/1e5`)

# Optimization analysis

   * `std::time::Duration` uses 96 bits -- u32 is just 1/3 of it;
   * `chrono`'s `NaiveDate` uses i32 -- u16 cuts it in half.

Adittionaly, serialization is aided by allowing representing date & times in native integer types.

# Usage example

```rust
use neat_date_time::neat_time;

let (h, m, s, ms, µs) = (17, 32, 42, 937, 0);
let expected_duration = std::time::Duration::from_micros(µs+(ms+(s+(m+h*60)*60)*1000)*1000);
let u32_duration = u32_from_24h_duration(duration);
dbg!(u32_duration);
let observed_duration = duration_from_24h_u32(u32_duration);
assert_eq!(observed_duration, expected_duration, "std duration <--> u32 conversions failed");
```

# Notes

This is the first version, just refactored out from existing code. The next version will have a better API for general use.