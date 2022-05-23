# neat-date-time crate

Dense & serializable representations of dates & times, providing
native types & operations to allow space efficient storage.


# Problem statement

Rust's `std` (as well as the `chrono` library) provides general date & time structures, meaning they offer:
   * general range
   * general precision

A storage price is payed for such generalization. Thus, there is a possible space optimization by fine tuning those two properties for domain specific needs.

# Application example

Lets take the example of stock markets. Trades can be grouped in daily sets and individual trades will happen between the oppening and closing time of the trading session. The possible optimizations are:
   * date range: by using a single `u16`, we're able to represent ~179 years -- by using an epoch date, any `std` or `chrono` date may be converted to `u16` back and forth;
   * time range & precision: we have two options here: use the full 24h range (with as much precision as possible) or use a partial range -- lets say, 12h -- at the double the precision. If we use a `u32` for time, a 24h range would allow a precision of ~20.117µs (or, precisely, `1/((2^32)/86400)*1e6`µs). On the other hand, if we want a precision of exactly 10µs, a u32 would be able to represent 11:55:49.67296s (from the formula `s*1e6 / (2^32) = µs_precision`, which resolves to `s=(2^32)/1e5`)

# Optimization analysis

   * `std::time::Duration` uses 96 bits -- `u32` is just 1/3 of it;
   * `chrono`'s `NaiveDate` uses `i32` -- `u16` cuts it in half.

A trading record consisting of a date, time, 8 chars asset, u32 unitary price and u32 quantity would require 68.75% of the original size -- a 16/11 relation.

If trades are grouped by asset and date, the size optimized version would require only 60% of the original size.

Adittionaly, serialization is aided by allowing representing dates & times as native integer types.

# Usage example

## Time
```rust
#![allow(uncommon_codepoints)]
use neat_date_time::neat_time;

let (h, m, s, ms, µs) = (17, 32, 42, 937, 3);
let expected_duration = std::time::Duration::from_micros(µs+(ms+(s+(m+h*60)*60)*1000)*1000);
let u32_duration = neat_time::u32_from_24h_duration(&expected_duration);
dbg!(u32_duration);
let observed_duration = neat_time::duration_from_24h_u32(u32_duration);
assert_eq!(observed_duration, expected_duration, "std duration <--> u32 conversions failed");
```

## `u32` date
```rust
use neat_date_time::neat_date;

let (original_year, original_month, original_day) = (1979, 01, 22);
let epoch = neat_date::u32_from_ymd(original_year as u16, original_month as u8, original_day as u8);
dbg!(epoch);
let (reconstructed_year, reconstructed_month, reconstructed_day) = neat_date::ymd_from_u32(epoch);
assert_eq!((reconstructed_year, reconstructed_month, reconstructed_day), (original_year, original_month, original_day), "naive dates <--> u32 conversions failed");
```


# Notes

This is the first version, just refactored out from existing code. The next version will have a better API for general use.

Currently, `u16` dates are representable as a delta from a `u32` date: just add or subtract the `u16` date to/from the `u32` absolute date.