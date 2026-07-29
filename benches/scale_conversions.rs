use anytime::{
    load_finals2000a,
    scales::{GPST, TAI, TCG, TDB, TT, UT1, UTC},
    Time,
};
use criterion::measurement::WallTime;
use criterion::{criterion_group, criterion_main, BenchmarkGroup, Criterion, Throughput};
use std::hint::black_box;

const REFERENCE_JD: f64 = 2_457_754.5;
const EOP_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/finals2000A.all");

fn benchmark_conversion<S, T>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    name: &'static str,
    input: Time<S>,
) where
    Time<S>: Into<Time<T>>,
{
    group.bench_function(name, |bencher| {
        bencher.iter_batched(
            || input.clone(),
            |value| {
                let converted: Time<T> = value.into();
                black_box(converted)
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

macro_rules! benchmark_pair {
    ($group:expr, $source:ty, $target:ty) => {
        benchmark_conversion::<$source, $target>(
            $group,
            concat!(stringify!($source), "_to_", stringify!($target)),
            Time::<$source>::from_jd(REFERENCE_JD),
        );
    };
}

fn scale_conversions(c: &mut Criterion) {
    // UT1 conversions use the loaded EOP data when the input is in range.
    load_finals2000a(EOP_PATH).expect("failed to load benchmark EOP data");

    let mut group = c.benchmark_group("scale_conversions");
    // Every benchmark performs exactly one scale conversion per iteration.
    group.throughput(Throughput::Elements(1));

    benchmark_pair!(&mut group, GPST, TAI);
    benchmark_pair!(&mut group, GPST, TCG);
    benchmark_pair!(&mut group, GPST, TDB);
    benchmark_pair!(&mut group, GPST, TT);
    benchmark_pair!(&mut group, GPST, UT1);
    benchmark_pair!(&mut group, GPST, UTC);

    benchmark_pair!(&mut group, TAI, GPST);
    benchmark_pair!(&mut group, TAI, TCG);
    benchmark_pair!(&mut group, TAI, TDB);
    benchmark_pair!(&mut group, TAI, TT);
    benchmark_pair!(&mut group, TAI, UT1);
    benchmark_pair!(&mut group, TAI, UTC);

    benchmark_pair!(&mut group, TCG, GPST);
    benchmark_pair!(&mut group, TCG, TAI);
    benchmark_pair!(&mut group, TCG, TDB);
    benchmark_pair!(&mut group, TCG, TT);
    benchmark_pair!(&mut group, TCG, UT1);
    benchmark_pair!(&mut group, TCG, UTC);

    benchmark_pair!(&mut group, TDB, GPST);
    benchmark_pair!(&mut group, TDB, TAI);
    benchmark_pair!(&mut group, TDB, TCG);
    benchmark_pair!(&mut group, TDB, TT);
    benchmark_pair!(&mut group, TDB, UT1);
    benchmark_pair!(&mut group, TDB, UTC);

    benchmark_pair!(&mut group, TT, GPST);
    benchmark_pair!(&mut group, TT, TAI);
    benchmark_pair!(&mut group, TT, TCG);
    benchmark_pair!(&mut group, TT, TDB);
    benchmark_pair!(&mut group, TT, UT1);
    benchmark_pair!(&mut group, TT, UTC);

    benchmark_pair!(&mut group, UT1, GPST);
    benchmark_pair!(&mut group, UT1, TAI);
    benchmark_pair!(&mut group, UT1, TCG);
    benchmark_pair!(&mut group, UT1, TDB);
    benchmark_pair!(&mut group, UT1, TT);
    benchmark_pair!(&mut group, UT1, UTC);

    benchmark_pair!(&mut group, UTC, GPST);
    benchmark_pair!(&mut group, UTC, TAI);
    benchmark_pair!(&mut group, UTC, TCG);
    benchmark_pair!(&mut group, UTC, TDB);
    benchmark_pair!(&mut group, UTC, TT);
    benchmark_pair!(&mut group, UTC, UT1);

    group.finish();
}

criterion_group!(benches, scale_conversions);
criterion_main!(benches);
