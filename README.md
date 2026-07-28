# anytime
Ergonomic astronomical rust timescales

`anytime` provides astronomical time scales and conversions with nanosecond
resolution. [`Time`](https://docs.rs/anytime/latest/anytime/struct.Time.html)
stores an instant with a type-level scale, making changes of scale explicit.
Supported scales are UTC, TAI, GPST, TT, TCG, TDB, and UT1.

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
