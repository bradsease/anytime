use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::sync::RwLock;

/// A measured or predicted Earth-orientation value from finals2000A.all.
///
/// Missing fields are represented by `None`. The angular quantities are in
/// arcseconds, UT1-UTC is in seconds, and LOD is in milliseconds, matching the
/// IERS fixed-width data product after it has been parsed.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Finals2000ARecord {
    /// Modified Julian Date of the record.
    mjd: f64,
    /// Polar motion in the x direction, in arcseconds.
    x_pole: Option<f64>,
    /// Uncertainty of [`Self::x_pole`], in arcseconds.
    x_pole_error: Option<f64>,
    /// Polar motion in the y direction, in arcseconds.
    y_pole: Option<f64>,
    /// Uncertainty of [`Self::y_pole`], in arcseconds.
    y_pole_error: Option<f64>,
    /// Whether the UT1-UTC value was observed or predicted.
    ut1_status: Option<DataStatus>,
    /// UT1-UTC, in seconds.
    ut1_minus_utc: Option<f64>,
    /// Uncertainty of [`Self::ut1_minus_utc`], in seconds.
    ut1_minus_utc_error: Option<f64>,
    /// Length-of-day correction, in milliseconds.
    lod: Option<f64>,
    /// Uncertainty of [`Self::lod`], in milliseconds.
    lod_error: Option<f64>,
}

/// Finals2000A records indexed directly by their consecutive integer MJD.
#[derive(Default)]
struct Finals2000AData {
    first_mjd: i64,
    records: Vec<Finals2000ARecord>,
}

/// Indicates whether an Earth-orientation value is observed or predicted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataStatus {
    /// The value was determined from observations.
    Observed,
    /// The value is an IERS prediction.
    Predicted,
}

static FINALS2000A_DATA: RwLock<Finals2000AData> = RwLock::new(Finals2000AData {
    first_mjd: 0,
    records: Vec::new(),
});

/// Errors encountered while loading finals2000A.all.
#[derive(Debug)]
pub enum FinalsLoadError {
    /// The input file could not be opened or read.
    Io(io::Error),
    /// A record could not be parsed.
    ///
    /// `line` is one-based and `message` describes the invalid field or
    /// structural condition.
    Parse {
        /// One-based input line containing the invalid record.
        line: usize,
        /// Description of the parse failure.
        message: String,
    },
    /// The shared dataset lock was poisoned by a panic in another thread.
    LockPoisoned,
}

impl fmt::Display for FinalsLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "could not read finals2000A file: {error}"),
            Self::Parse { line, message } => {
                write!(
                    formatter,
                    "invalid finals2000A record on line {line}: {message}"
                )
            }
            Self::LockPoisoned => write!(formatter, "finals2000A data lock is poisoned"),
        }
    }
}

impl Error for FinalsLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Parse { .. } | Self::LockPoisoned => None,
        }
    }
}

/// Errors encountered while sampling UT1-UTC data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EopError {
    /// The shared dataset lock was poisoned by a panic in another thread.
    LockPoisoned,
    /// No dataset has been loaded, or it contains no UT1-UTC values.
    Empty,
    /// The requested MJD is outside the loaded data range.
    OutOfRange,
    /// The requested MJD is not finite.
    InvalidMjd,
}

impl fmt::Display for EopError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::LockPoisoned => "finals2000A data lock is poisoned",
            Self::Empty => "no UT1-UTC data has been loaded",
            Self::OutOfRange => "MJD is outside the loaded UT1-UTC range",
            Self::InvalidMjd => "MJD must be finite",
        };
        formatter.write_str(message)
    }
}

impl Error for EopError {}

/// Load and atomically replace the shared finals2000A dataset.
///
/// The file is parsed completely before the existing dataset is replaced, so
/// a parsing or I/O error leaves previously loaded data available. The return
/// value is the number of records loaded. Records must have consecutive,
/// integer MJD values, as required by the finals2000A daily data product.
///
/// # Examples
///
/// ```no_run
/// use anytime::load_finals2000a;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let records = load_finals2000a("data/finals2000A.all")?;
/// println!("loaded {records} Earth-orientation records");
/// # Ok(())
/// # }
/// ```
pub fn load_finals2000a<P>(path: P) -> Result<usize, FinalsLoadError>
where
    P: AsRef<Path>,
{
    let file = File::open(path).map_err(FinalsLoadError::Io)?;
    load_finals2000a_reader(BufReader::new(file))
}

fn load_finals2000a_reader<R>(reader: R) -> Result<usize, FinalsLoadError>
where
    R: BufRead,
{
    let parsed_data = parse_finals2000a(reader)?;
    let count = parsed_data.records.len();

    let mut data = FINALS2000A_DATA
        .write()
        .map_err(|_| FinalsLoadError::LockPoisoned)?;
    *data = parsed_data;

    Ok(count)
}

/// Interpolate an internal UT1-UTC value for an MJD.
pub(crate) fn sample_ut1_minus_utc(mjd: f64) -> Result<f64, EopError> {
    let data = FINALS2000A_DATA
        .read()
        .map_err(|_| EopError::LockPoisoned)?;
    sample_ut1_minus_utc_from_data(&data, mjd)
}

fn sample_ut1_minus_utc_from_data(data: &Finals2000AData, mjd: f64) -> Result<f64, EopError> {
    if !mjd.is_finite() {
        return Err(EopError::InvalidMjd);
    }

    if data.records.is_empty() {
        return Err(EopError::Empty);
    }

    let first_mjd = data.first_mjd as f64;
    let end_mjd = first_mjd + data.records.len() as f64;
    if mjd < first_mjd || mjd >= end_mjd {
        return Err(EopError::OutOfRange);
    }

    let index = (mjd.floor() as i64 - data.first_mjd) as usize;
    let previous = data.records[..=index]
        .iter()
        .rev()
        .find(|record| record.ut1_minus_utc.is_some());

    let Some(previous) = previous else {
        let next = data.records[index + 1..]
            .iter()
            .find(|record| record.ut1_minus_utc.is_some());
        return if next.is_some() {
            Err(EopError::OutOfRange)
        } else {
            Err(EopError::Empty)
        };
    };

    let previous_value = previous
        .ut1_minus_utc
        .expect("filtered previous EOP record has UT1-UTC");
    if previous.mjd == mjd {
        return Ok(previous_value);
    }

    let next = data.records[index + 1..]
        .iter()
        .find(|record| record.ut1_minus_utc.is_some())
        .ok_or(EopError::OutOfRange)?;
    let next_value = next
        .ut1_minus_utc
        .expect("filtered next EOP record has UT1-UTC");
    let fraction = (mjd - previous.mjd) / (next.mjd - previous.mjd);
    let mut delta = next_value - previous_value;
    delta -= delta.round();
    Ok(previous_value + fraction * delta)
}

fn parse_finals2000a<R>(reader: R) -> Result<Finals2000AData, FinalsLoadError>
where
    R: BufRead,
{
    let mut records = Vec::new();
    let mut first_mjd = 0;
    let mut previous_mjd: Option<i64> = None;

    for (line_index, line) in reader.lines().enumerate() {
        let line_number = line_index + 1;
        let line = line.map_err(FinalsLoadError::Io)?;
        if line.trim().is_empty() {
            continue;
        }

        let record = parse_record(&line, line_number)?;
        let mjd = record.mjd as i64;
        if record.mjd != mjd as f64 {
            return Err(FinalsLoadError::Parse {
                line: line_number,
                message: "MJD values must be integers".to_owned(),
            });
        }
        if let Some(previous_mjd) = previous_mjd {
            if previous_mjd.checked_add(1) != Some(mjd) {
                return Err(FinalsLoadError::Parse {
                    line: line_number,
                    message: "MJD values must be consecutive days".to_owned(),
                });
            }
        } else {
            first_mjd = mjd;
        }
        previous_mjd = Some(mjd);
        records.push(record);
    }

    Ok(Finals2000AData { first_mjd, records })
}

fn parse_record(line: &str, line_number: usize) -> Result<Finals2000ARecord, FinalsLoadError> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    let mjd_index = fields
        .iter()
        .position(|field| field.contains('.') && field.parse::<f64>().is_ok())
        .ok_or_else(|| parse_error(line_number, "missing MJD"))?;
    let mjd = parse_value(fields[mjd_index], line_number, "MJD")?;
    if !mjd.is_finite() {
        return Err(parse_error(line_number, "MJD must be finite"));
    }

    let values = normalize_fields(&fields[mjd_index + 1..]);
    if values.is_empty() {
        return Ok(Finals2000ARecord {
            mjd,
            x_pole: None,
            x_pole_error: None,
            y_pole: None,
            y_pole_error: None,
            ut1_status: None,
            ut1_minus_utc: None,
            ut1_minus_utc_error: None,
            lod: None,
            lod_error: None,
        });
    }

    let ut1_status = values
        .get(5)
        .map(|value| parse_status(value, line_number))
        .transpose()?;

    let lod = parse_optional_value_or_missing(values.get(8), line_number, "LOD")?;
    let lod_error = if lod.is_some() {
        parse_optional_value(values.get(9), line_number, "LOD error")?
    } else {
        None
    };

    Ok(Finals2000ARecord {
        mjd,
        x_pole: parse_optional_value(values.get(1), line_number, "x pole")?,
        x_pole_error: parse_optional_value(values.get(2), line_number, "x pole error")?,
        y_pole: parse_optional_value(values.get(3), line_number, "y pole")?,
        y_pole_error: parse_optional_value(values.get(4), line_number, "y pole error")?,
        ut1_status,
        ut1_minus_utc: parse_optional_value(values.get(6), line_number, "UT1-UTC")?,
        ut1_minus_utc_error: parse_optional_value(values.get(7), line_number, "UT1-UTC error")?,
        lod,
        lod_error,
    })
}

fn parse_status(value: &str, line_number: usize) -> Result<DataStatus, FinalsLoadError> {
    match value {
        "I" => Ok(DataStatus::Observed),
        "P" => Ok(DataStatus::Predicted),
        _ => Err(parse_error(line_number, "status must be I or P")),
    }
}

fn normalize_fields(fields: &[&str]) -> Vec<String> {
    let mut normalized = Vec::with_capacity(fields.len());

    for field in fields {
        let first = field.as_bytes().first().copied();
        let glued_status = matches!(first, Some(b'I' | b'P'))
            && field.len() > 1
            && field[1..].parse::<f64>().is_ok();

        if glued_status {
            normalized.push(field[..1].to_owned());
            normalized.push(field[1..].to_owned());
        } else {
            normalized.push((*field).to_owned());
        }
    }

    normalized
}

fn parse_optional_value(
    value: Option<&String>,
    line_number: usize,
    field_name: &str,
) -> Result<Option<f64>, FinalsLoadError> {
    value
        .map(|value| parse_value(value, line_number, field_name))
        .transpose()
}

fn parse_optional_value_or_missing(
    value: Option<&String>,
    line_number: usize,
    field_name: &str,
) -> Result<Option<f64>, FinalsLoadError> {
    match value {
        None => Ok(None),
        Some(value) if value == "I" || value == "P" => Ok(None),
        Some(value) => Ok(Some(parse_value(value, line_number, field_name)?)),
    }
}

fn parse_value(value: &str, line_number: usize, field_name: &str) -> Result<f64, FinalsLoadError> {
    value.parse::<f64>().map_err(|_| {
        parse_error(
            line_number,
            &format!("{field_name} is not a floating-point value"),
        )
    })
}

fn parse_error(line: usize, message: &str) -> FinalsLoadError {
    FinalsLoadError::Parse {
        line,
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::sync::{Mutex, OnceLock};
    use std::thread;

    const EXAMPLE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

    fn global_test_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn parses_the_preloaded_example_file() {
        let contents = include_str!("../data/finals2000A.all");
        let data = parse_finals2000a(Cursor::new(contents)).unwrap();

        assert_eq!(data.first_mjd, 41_684);
        assert_eq!(data.records.len(), 19_977);
        assert_eq!(data.records[0].mjd, 41_684.0);
        assert_eq!(data.records[0].ut1_minus_utc, Some(0.8084178));
        assert_eq!(data.records[0].ut1_status, Some(DataStatus::Observed));
        assert_eq!(data.records[1].mjd, 41_685.0);
        assert_eq!(data.records[1].ut1_minus_utc, Some(0.8056163));
        assert_eq!(data.records[19_976].mjd, 61_660.0);
        assert_eq!(data.records[19_976].ut1_minus_utc, None);
    }

    #[test]
    fn loads_the_example_file_into_the_shared_store() {
        let _lock = global_test_lock().lock().unwrap();
        let count = load_finals2000a(EXAMPLE_PATH).unwrap();
        assert_eq!(count, 19_977);

        let data = FINALS2000A_DATA.read().unwrap();
        assert_eq!(data.records.len(), count);
        assert_eq!(data.records[0].x_pole, Some(0.120733));
        assert_eq!(data.records[0].y_pole, Some(0.136966));
    }

    #[test]
    fn samples_the_example_file_at_a_record() {
        let _lock = global_test_lock().lock().unwrap();
        load_finals2000a(EXAMPLE_PATH).unwrap();

        assert_eq!(sample_ut1_minus_utc(41_684.0).unwrap(), 0.8084178);
        assert_eq!(sample_ut1_minus_utc(41_685.0).unwrap(), 0.8056163);
        let midpoint = sample_ut1_minus_utc(41_684.5).unwrap();
        assert!((midpoint - 0.80701705).abs() < 1e-12);
        let leap_second_midpoint = sample_ut1_minus_utc(42_047.5).unwrap();
        assert!((leap_second_midpoint - -0.2985993).abs() < 1e-12);
        assert_eq!(sample_ut1_minus_utc(61_660.0), Err(EopError::OutOfRange));
    }

    #[test]
    fn matches_reference_data() {
        let _lock = global_test_lock().lock().unwrap();
        load_finals2000a(EXAMPLE_PATH).unwrap();

        for line in include_str!("../data/finals2000a_references.txt").lines() {
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }

            let mut fields = line.split_whitespace();
            let mjd = fields.next().unwrap().parse::<f64>().unwrap();
            let expected = fields.next().unwrap().parse::<f64>().unwrap();
            let actual = sample_ut1_minus_utc(mjd).unwrap();

            assert!(
                (actual - expected).abs() < 1e-12,
                "MJD {mjd}: expected {expected}, got {actual}"
            );
        }
    }

    fn test_record(mjd: f64, ut1_minus_utc: Option<f64>) -> Finals2000ARecord {
        Finals2000ARecord {
            mjd,
            x_pole: None,
            x_pole_error: None,
            y_pole: None,
            y_pole_error: None,
            ut1_status: ut1_minus_utc.map(|_| DataStatus::Observed),
            ut1_minus_utc,
            ut1_minus_utc_error: None,
            lod: None,
            lod_error: None,
        }
    }

    #[test]
    fn interpolates_between_records() {
        let data = Finals2000AData {
            first_mjd: 100,
            records: vec![
                test_record(100.0, Some(1.0)),
                test_record(101.0, None),
                test_record(102.0, Some(1.2)),
            ],
        };

        assert_eq!(sample_ut1_minus_utc_from_data(&data, 100.0), Ok(1.0));
        assert_eq!(sample_ut1_minus_utc_from_data(&data, 101.0), Ok(1.1));
        assert_eq!(sample_ut1_minus_utc_from_data(&data, 101.5), Ok(1.15));
        assert_eq!(
            sample_ut1_minus_utc_from_data(&data, 99.0),
            Err(EopError::OutOfRange)
        );
        assert_eq!(
            sample_ut1_minus_utc_from_data(&data, 103.0),
            Err(EopError::OutOfRange)
        );
    }

    #[test]
    fn rejects_non_daily_records() {
        let non_integer = parse_finals2000a(Cursor::new("73 1 2 41684.50"));
        assert!(matches!(
            non_integer,
            Err(FinalsLoadError::Parse { line: 1, .. })
        ));

        let gap = parse_finals2000a(Cursor::new("73 1 2 41684.00\n73 1 2 41686.00"));
        assert!(matches!(gap, Err(FinalsLoadError::Parse { line: 2, .. })));
    }

    #[test]
    fn rejects_invalid_input_without_replacing_existing_data() {
        let _lock = global_test_lock().lock().unwrap();
        load_finals2000a(EXAMPLE_PATH).unwrap();
        let result = load_finals2000a_reader(Cursor::new("73 1 2 41684.00 I not-a-number"));

        assert!(matches!(
            result,
            Err(FinalsLoadError::Parse { line: 1, .. })
        ));
        assert_eq!(sample_ut1_minus_utc(41_684.0).unwrap(), 0.8084178);
    }

    #[test]
    fn rejects_non_finite_sample_points() {
        let _lock = global_test_lock().lock().unwrap();
        assert_eq!(
            sample_ut1_minus_utc_from_data(&Finals2000AData::default(), f64::NAN),
            Err(EopError::InvalidMjd)
        );
        assert_eq!(sample_ut1_minus_utc(f64::NAN), Err(EopError::InvalidMjd));
    }

    #[test]
    fn supports_concurrent_sampling() {
        let _lock = global_test_lock().lock().unwrap();
        load_finals2000a(EXAMPLE_PATH).unwrap();

        let workers: Vec<_> = (0..8)
            .map(|worker| {
                thread::spawn(move || {
                    let mjd = 41_684.0 + worker as f64 * 0.25;
                    sample_ut1_minus_utc(mjd).unwrap()
                })
            })
            .collect();

        for worker in workers {
            assert!(worker.join().unwrap().is_finite());
        }
    }
}
