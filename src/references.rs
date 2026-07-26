use crate::scales::{GPST, TAI, TCG, TT, UT1, UTC};
use crate::{load_finals2000a, Scale, Time};
use chrono::NaiveDateTime;

const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");
const REFERENCES: &str = include_str!("../data/time_scales_references.txt");
const MAX_ERROR_NANOSECONDS: i64 = 1;

#[derive(Debug)]
struct Representations {
    split_jd: (f64, f64),
    gregorian: NaiveDateTime,
}

#[derive(Debug)]
struct ReferenceCase {
    gpst: Representations,
    tai: Representations,
    tcg: Representations,
    tt: Representations,
    ut1: Representations,
    utc: Representations,
}

fn representations(fields: &[&str]) -> Representations {
    Representations {
        split_jd: (
            fields[0].parse().expect("reference JD1 must be an f64"),
            fields[1].parse().expect("reference JD2 must be an f64"),
        ),
        gregorian: NaiveDateTime::parse_from_str(fields[2], "%Y-%m-%dT%H:%M:%S%.f")
            .expect("reference Gregorian value must be an ISO date and time"),
    }
}

fn cases() -> Vec<ReferenceCase> {
    REFERENCES
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .map(|line| {
            let fields: Vec<_> = line.split_whitespace().collect();
            assert_eq!(
                fields.len(),
                18,
                "reference case must have 18 values: {line}"
            );

            ReferenceCase {
                gpst: representations(&fields[0..3]),
                tai: representations(&fields[3..6]),
                tcg: representations(&fields[6..9]),
                tt: representations(&fields[9..12]),
                ut1: representations(&fields[12..15]),
                utc: representations(&fields[15..18]),
            }
        })
        .collect()
}

fn assert_split_jd<S: Scale>(
    actual: &Time<S>,
    expected: &Representations,
    case: &ReferenceCase,
    target: &str,
) {
    let expected = Time::<S>::from_split_jd(expected.split_jd.0, expected.split_jd.1);
    let error = actual.value - expected.value;
    let error_nanoseconds = error.num_seconds() * 1_000_000_000 + error.subsec_nanos() as i64;

    assert!(
        error_nanoseconds.abs() <= MAX_ERROR_NANOSECONDS,
        "UTC JD {} + {} converted to {target} with an error of {error_nanoseconds} ns",
        case.utc.split_jd.0,
        case.utc.split_jd.1,
    );
}

fn assert_representations<S: Scale>(
    actual: Time<S>,
    expected: &Representations,
    case: &ReferenceCase,
    target: &str,
) {
    assert_split_jd(&actual, expected, case, target);
    let error = actual.gregorian() - expected.gregorian;
    let error_nanoseconds = error.num_seconds() * 1_000_000_000 + error.subsec_nanos() as i64;
    assert!(
        error_nanoseconds.abs() <= MAX_ERROR_NANOSECONDS,
        "UTC JD {} + {} converted to a {target} Gregorian value with an error of \
         {error_nanoseconds} ns",
        case.utc.split_jd.0,
        case.utc.split_jd.1,
    );
}

macro_rules! assert_conversions {
    ($time:expr, $case:expr) => {{
        let time = $time;
        assert_representations(time.clone().gpst(), &$case.gpst, $case, "GPST");
        assert_representations(time.clone().tai(), &$case.tai, $case, "TAI");
        assert_representations(time.clone().tcg(), &$case.tcg, $case, "TCG");
        assert_representations(time.clone().tt(), &$case.tt, $case, "TT");
        assert_representations(time.clone().ut1(), &$case.ut1, $case, "UT1");
        assert_representations(time.utc(), &$case.utc, $case, "UTC");
    }};
}

macro_rules! assert_source_representations {
    ($scale:ty, $expected:expr, $case:expr) => {{
        let expected = $expected;
        assert_conversions!(
            Time::<$scale>::from_split_jd(expected.split_jd.0, expected.split_jd.1),
            $case
        );
        assert_conversions!(Time::<$scale>::from_gregorian(expected.gregorian), $case);
    }};
}

#[test]
fn matches_reference_cases_for_every_scale_conversion() {
    load_finals2000a(EXAMPLE_PATH).unwrap();

    for case in cases() {
        assert_source_representations!(GPST, &case.gpst, &case);
        assert_source_representations!(TAI, &case.tai, &case);
        assert_source_representations!(TCG, &case.tcg, &case);
        assert_source_representations!(TT, &case.tt, &case);
        assert_source_representations!(UT1, &case.ut1, &case);
        assert_source_representations!(UTC, &case.utc, &case);
    }
}
