use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use generic_array::{typenum::*, ArrayLength, GenericArray};
use rand::RngCore;

use std::{fmt::UpperHex, io::Write};

fn criterion_benchmark(c: &mut Criterion) {
    let mut hex = c.benchmark_group("hex");

    let mut rng = rand::rng();

    macro_rules! all_hex_benches {
        ($($len:ty,)*) => {
            $(bench_hex::<$len>(&mut rng, &mut hex);)*
        }
    }

    all_hex_benches!(
        U1, U2, U4, U8, U12, U15, U16, U32, U64, U100, U128, U160, U255, U256, U500, U512, U900,
        U1023, U1024, Sum<U1024, U1>, U2048, U4096, Prod<U1000, U5>, U10000,
    );

    hex.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn bench_hex<N: ArrayLength>(mut rng: impl RngCore, g: &mut BenchmarkGroup<'_, WallTime>)
where
    GenericArray<u8, N>: UpperHex,
{
    let mut fixture = Box::<GenericArray<u8, N>>::default();
    rng.fill_bytes(fixture.as_mut_slice());

    g.bench_function(format!("N{:08}", N::USIZE), |b| {
        let mut out = Vec::with_capacity(N::USIZE * 2);

        b.iter(|| {
            _ = write!(out, "{:X}", &*fixture);
            out.clear();
        });
    });
}
