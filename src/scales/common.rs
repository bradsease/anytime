use crate::constants::{DAY_SECONDS, JD_TO_MJD, TAI_UTC};

pub(crate) struct UtcDay {
    pub(crate) midnight_jd: f64,
    pub(crate) scaled_fractional_day: f64,
    pub(crate) fractional_day_scale: f64,
    pub(crate) tai_minus_utc: f64,
}

pub(crate) fn utc_day(jd1: f64, jd2: f64) -> UtcDay {
    let (midnight_jd, fractional_day) = if jd2 >= 0.5 {
        (jd1 + 0.5, jd2 - 0.5)
    } else {
        (jd1 - 0.5, jd2 + 0.5)
    };

    let mut today_entry = TAI_UTC[0];
    for entry in TAI_UTC.iter().rev() {
        if midnight_jd >= entry.0 {
            today_entry = *entry;
            break;
        }
    }
    let today_offset = today_entry.1 + (midnight_jd - JD_TO_MJD - today_entry.2) * today_entry.3;
    let midday_offset =
        today_entry.1 + (midnight_jd + 0.5 - JD_TO_MJD - today_entry.2) * today_entry.3;

    let mut tomorrow_entry = TAI_UTC[0];
    for entry in TAI_UTC.iter().rev() {
        if midnight_jd + 1.0 >= entry.0 {
            tomorrow_entry = *entry;
            break;
        }
    }
    let tomorrow_offset =
        tomorrow_entry.1 + (midnight_jd + 1.0 - JD_TO_MJD - tomorrow_entry.2) * tomorrow_entry.3;

    let daily_drift = 2.0 * (midday_offset - today_offset);
    let boundary_jump = tomorrow_offset - (today_offset + daily_drift);
    let fractional_day_scale =
        (DAY_SECONDS + boundary_jump) / DAY_SECONDS * (DAY_SECONDS + daily_drift) / DAY_SECONDS;
    let scaled_fractional_day = fractional_day * fractional_day_scale;

    UtcDay {
        midnight_jd,
        scaled_fractional_day,
        fractional_day_scale,
        tai_minus_utc: today_offset + (scaled_fractional_day - fractional_day) * DAY_SECONDS,
    }
}

#[cfg(test)]
pub(crate) fn assert_round_trip<S, T>(time: crate::Time<S>)
where
    S: Clone,
    crate::Time<T>: From<crate::Time<S>>,
    crate::Time<S>: From<crate::Time<T>>,
{
    use chrono::TimeDelta;

    let original = time.clone();
    let converted: crate::Time<T> = time.into();
    let round_trip: crate::Time<S> = converted.into();
    let error = round_trip.value - original.value;
    let absolute_error = if error < TimeDelta::seconds(0) {
        -error
    } else {
        error
    };

    let error_ns = absolute_error.num_seconds() as f64 * 1e9 + absolute_error.subsec_nanos() as f64;
    assert!(error_ns <= 0.1, "round-trip error was {error_ns} ns");
}
