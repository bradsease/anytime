#!/usr/bin/env python3
"""Generate Astropy reference cases for the core time scales exposed by anytime.

The UT1 values use only the UT1_UTC_A column from the repository's
finals2000A.all file. IERS-B data is neither read nor used.
"""

import argparse
from pathlib import Path

import numpy as np
from astropy.table import QTable
from astropy.time import Time, TimeDelta
from astropy.utils import iers


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_INPUT = ROOT / "data" / "finals2000A.all"
DEFAULT_OUTPUT = ROOT / "data" / "time_scales_references.txt"
DEFAULT_CASE_COUNT = 1000
JD_TO_MJD = 2_400_000.5
SCALE_NAMES = ("TAI", "TCB", "TCG", "TDB", "TT", "UT1", "UTC")

LEAP_SECOND_DATES = (
    "1973-12-31",
    "1974-12-31",
    "1975-12-31",
    "1976-12-31",
    "1977-12-31",
    "1978-12-31",
    "1979-12-31",
    "1981-06-30",
    "1982-06-30",
    "1983-06-30",
    "1985-06-30",
    "1987-12-31",
    "1989-12-31",
    "1990-12-31",
    "1992-06-30",
    "1993-06-30",
    "1994-06-30",
    "1995-12-31",
    "1997-06-30",
    "1998-12-31",
    "2005-12-31",
    "2008-12-31",
    "2012-06-30",
    "2015-06-30",
    "2016-12-31",
)
# The finals file starts on 1973-01-02, so the 1972 leap seconds cannot
# produce valid A-only UT1 reference values.


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--input", type=Path, default=DEFAULT_INPUT)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    parser.add_argument(
        "--count",
        type=int,
        default=DEFAULT_CASE_COUNT,
        help=f"number of cases to generate (default: {DEFAULT_CASE_COUNT})",
    )
    parser.add_argument(
        "--start-mjd",
        type=float,
        help="first UTC MJD to include (default: first A-only EOP sample)",
    )
    parser.add_argument(
        "--end-mjd",
        type=float,
        help="last UTC MJD to include (default: last A-only EOP sample)",
    )
    return parser.parse_args()


def load_a_only_eop(path):
    source = iers.IERS_A.read(path)
    valid = np.isfinite(source["UT1_UTC_A"])
    table = iers.IERS(
        QTable(
            {
                "MJD": source["MJD"][valid],
                "UT1_UTC": source["UT1_UTC_A"][valid],
            }
        )
    )
    return table


def leap_second_samples():
    samples = []
    for date in LEAP_SECOND_DATES:
        # Bracket the start of the inserted leap second with 1 ns margins.
        before = Time(f"{date}T23:59:59.999999999", format="isot", scale="utc")
        after = before + TimeDelta(2e-9, format="sec")
        samples.extend((before, after))
    return samples


def choose_times(count, start_mjd, end_mjd):
    if count < 2:
        raise ValueError("--count must be at least 2 so the range endpoints are included")
    if start_mjd >= end_mjd:
        raise ValueError("--start-mjd must be earlier than --end-mjd")

    selected = []
    selected_keys = set()

    def add(time):
        key = (float(time.jd1), float(time.jd2))
        if key not in selected_keys:
            selected.append(time)
            selected_keys.add(key)

    add(Time(JD_TO_MJD + start_mjd, format="jd", scale="utc"))
    add(Time(JD_TO_MJD + end_mjd, format="jd", scale="utc"))
    for time in leap_second_samples():
        if start_mjd <= time.mjd <= end_mjd:
            add(time)

    if len(selected) > count:
        raise ValueError(
            f"--count={count} is too small to retain {len(selected)} endpoint and "
            "leap-second cases"
        )

    # Add evenly spaced dates across the full range, retaining the endpoints
    # and leap-second cases above exactly.
    remaining = count - len(selected)
    for mjd in np.linspace(start_mjd, end_mjd, remaining + 2)[1:-1]:
        add(Time(JD_TO_MJD + mjd, format="jd", scale="utc"))

    # A requested date can coincide with a leap-second case. Fill any such
    # gaps with a denser uniform sample while retaining the broad coverage.
    for mjd in np.linspace(start_mjd, end_mjd, count * 4):
        if len(selected) == count:
            break
        add(Time(JD_TO_MJD + mjd, format="jd", scale="utc"))

    if len(selected) != count:
        raise RuntimeError("could not select the requested number of distinct cases")
    return sorted(selected, key=lambda time: (time.jd1, time.jd2))


def find_usable_boundary(mjds, reverse=False):
    for mjd in sorted(mjds, reverse=reverse):
        try:
            Time(JD_TO_MJD + mjd, format="jd", scale="utc").ut1
        except iers.IERSRangeError:
            continue
        return float(mjd)
    raise RuntimeError("the A-only IERS table has no usable UT1 samples")


def split_jd(time):
    return time.jd1, time.jd2


def gregorian(time):
    time.precision = 9
    return time.to_value("isot", subfmt="date_hms")


def scale_times(utc):
    tai = utc.tai
    return {
        "TAI": tai,
        "TCB": utc.tcb,
        "TCG": utc.tcg,
        "TDB": utc.tdb,
        "TT": utc.tt,
        "UT1": utc.ut1,
        "UTC": utc,
    }


def format_value(value):
    return f"{value:.17g}"


def main():
    args = parse_args()
    if not args.input.is_file():
        raise FileNotFoundError(f"IERS finals file does not exist: {args.input}")

    try:
        input_display = args.input.resolve().relative_to(ROOT)
    except ValueError:
        input_display = args.input

    a_only_eop = load_a_only_eop(args.input)
    available_mjds = np.asarray(a_only_eop["MJD"], dtype=float)

    # Suppress Astropy's automatic EOP lookup and install an A-only table for
    # the duration of every UT1 conversion.
    with iers.conf.set_temp("auto_download", False):
        with iers.earth_orientation_table.set(a_only_eop):
            first_mjd = find_usable_boundary(available_mjds)
            last_mjd = find_usable_boundary(available_mjds, reverse=True)
            start_mjd = first_mjd if args.start_mjd is None else args.start_mjd
            end_mjd = last_mjd if args.end_mjd is None else args.end_mjd
            if start_mjd < first_mjd or end_mjd > last_mjd:
                raise ValueError(
                    f"requested range [{start_mjd}, {end_mjd}] is outside A-only coverage "
                    f"[{first_mjd}, {last_mjd}]"
                )

            times = choose_times(args.count, start_mjd, end_mjd)
            lines = [
                "# Generated by validation/generate_time_scales_astropy.py",
                f"# Input: {input_display}",
                "# UT1 uses finals2000A UT1_UTC_A only; IERS-B data is not used.",
                "# "
                + " ".join(
                    field
                    for scale in SCALE_NAMES
                    for field in (f"{scale}_JD1", f"{scale}_JD2", f"{scale}_GREGORIAN")
                ),
            ]
            for utc in times:
                scales = scale_times(utc)
                values = []
                for scale in SCALE_NAMES:
                    values.extend(format_value(value) for value in split_jd(scales[scale]))
                    values.append(gregorian(scales[scale]))
                lines.append(" ".join(values))

    args.output.write_text("\n".join(lines) + "\n")
    print(f"wrote {args.output} ({len(times)} cases)")


if __name__ == "__main__":
    main()
