# anytime
Ergonomic astronomical rust timescales

`anytime` provides astronomical time scales and conversions with nanosecond
resolution. [`Time`](https://docs.rs/anytime/latest/anytime/struct.Time.html)
stores an instant with a type-level scale, making changes of scale explicit.
Supported scales are UTC, TAI, GPST, TT, TCG, TCB, TDB, and UT1.

Enable the optional `serde` feature to serialize and deserialize `Time`,
`AnyTime`, `TimeSeries`, and `TimeScale` values.

## Examples

### Convert between scales

```rust
use anytime::{scales::{TAI, UTC}, Time};

let utc = Time::<UTC>::from_jd(2_451_545.0);
let tai: Time<TAI> = utc.into();
```

### Compare and difference

Values can be compared and differenced directly, even when they use different
scales. Subtraction returns a `chrono::TimeDelta`.

```rust
use anytime::{scales::{TAI, UTC}, Time};

let earlier = Time::<UTC>::from_jd(2_451_545.0);
let later: Time<TAI> = Time::<UTC>::from_jd(2_451_546.0).into();
let is_ordered = earlier < later;
let difference = later - earlier;
```

### Format a Gregorian date

`gregorian().to_string()` preserves sub-second precision.

```rust
use anytime::{scales::UTC, Time};
use chrono::NaiveDate;

let date = NaiveDate::from_ymd_opt(2000, 1, 1)
    .unwrap()
    .and_hms_nano_opt(12, 0, 0, 123_456_789)
    .unwrap();
let date_string = Time::<UTC>::from_gregorian(date)
    .gregorian()
    .to_string();
assert_eq!(date_string, "2000-01-01 12:00:00.123456789");
```

### Construct with a runtime scale

```rust
use anytime::{AnyTime, TimeScale};

let utc = AnyTime::from_isot_str("2000-01-01T12:00:00", TimeScale::UTC).unwrap();
assert_eq!(utc, AnyTime::from_jd(2_451_545.0, TimeScale::UTC));
```

### Parse a typed time

`Time<S>` provides the same parsing constructors as `AnyTime`. Its `FromStr`
implementation assumes an ISO 8601 `T`-separated date and time without an
offset, interpreted in `S`.

```rust
use anytime::{scales::UTC, Time};

let utc: Time<UTC> = "2000-01-01T12:00:00".parse().unwrap();
assert_eq!(utc, Time::<UTC>::from_jd(2_451_545.0));
```

## Benchmarks

Run the scale conversion benchmarks with Criterion:

```bash
cargo bench --bench scale_conversions
```

To compare each conversion with the fastest result, run:

```bash
python3 scripts/benchmark_relative_cost.py
```

The helper reads Criterion's median estimates and reports relative cost, where
the fastest conversion is `1.000x`.
