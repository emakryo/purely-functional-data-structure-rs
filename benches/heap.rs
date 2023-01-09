use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, AxisScale, BenchmarkGroup,
    BenchmarkId, Criterion, PlotConfiguration, Throughput,
};
use purely_functional_data_structure::heap::{
    binomial_heap::BinomialHeap, lazy_binomial_heap::LazyBinomialHeap,
    lazy_pairing_heap::LazyPairingHeap, leftist_heap::LeftistHeap, pairing_heap::PairingHeap,
    scheduled_binomial_heap::ScheduledBinomialHeap, splay_heap::SplayHeap, Heap,
};
use rand::prelude::*;

fn insert<H: Heap<u64>>(xs: &[u64]) -> H {
    xs.iter().fold(H::empty(), |h, &x| h.insert(x))
}

fn gen_random(n: usize) -> Vec<u64> {
    let mut xs = (0..n as u64).collect::<Vec<_>>();
    let mut rng = StdRng::seed_from_u64(100);
    xs.shuffle(&mut rng);
    xs
}

fn benchmark_insert(g: &mut BenchmarkGroup<WallTime>, xs: &[u64]) {
    g.throughput(Throughput::Elements(xs.len() as u64));
    macro_rules! bn {
        ($heap:ident) => {
            g.bench_with_input(
                BenchmarkId::new(stringify!($heap), xs.len()),
                &xs,
                |b, xs| b.iter(|| insert::<$heap<u64>>(black_box(xs))),
            );
        };
    }
    bn!(LeftistHeap);
    bn!(BinomialHeap);
    bn!(PairingHeap);
    bn!(SplayHeap);
    bn!(LazyBinomialHeap);
    bn!(LazyPairingHeap);
    bn!(ScheduledBinomialHeap);
}

fn benchmark_find_min(g: &mut BenchmarkGroup<WallTime>, xs: &[u64]) {
    g.throughput(Throughput::Elements(xs.len() as u64));
    macro_rules! bn {
        ($heap:ident) => {
            let h = insert::<$heap<u64>>(xs);
            g.bench_with_input(BenchmarkId::new(stringify!($heap), xs.len()), &h, |b, h| {
                b.iter(|| h.find_min())
            });
        };
    }
    bn!(LeftistHeap);
    bn!(BinomialHeap);
    bn!(PairingHeap);
    bn!(SplayHeap);
    bn!(LazyBinomialHeap);
    bn!(LazyPairingHeap);
    bn!(ScheduledBinomialHeap);
}

fn benchmark_delete_min(g: &mut BenchmarkGroup<WallTime>, xs: &[u64]) {
    g.throughput(Throughput::Elements(xs.len() as u64));
    macro_rules! bn {
        ($heap:ident) => {
            let h = insert::<$heap<u64>>(xs);
            g.bench_with_input(BenchmarkId::new(stringify!($heap), xs.len()), &h, |b, h| {
                b.iter(|| h.delete_min())
            });
        };
    }
    bn!(LeftistHeap);
    bn!(BinomialHeap);
    bn!(PairingHeap);
    bn!(SplayHeap);
    bn!(LazyBinomialHeap);
    bn!(LazyPairingHeap);
    bn!(ScheduledBinomialHeap);
}

pub fn benchmark_heaps(c: &mut Criterion) {
    let mut g = c.benchmark_group("insert_asc_sorted");
    g.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for n in [10, 100, 1000, 10000, 100000] {
        let xs = (0..n).collect::<Vec<_>>();
        benchmark_insert(&mut g, &xs);
    }
    g.finish();

    let mut g = c.benchmark_group("insert_random");
    g.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for n in [10, 100, 1000, 10000, 100000] {
        let xs = gen_random(n);
        benchmark_insert(&mut g, &xs);
    }
    g.finish();

    let mut g = c.benchmark_group("find_min_asc_sorted");
    g.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for n in [10, 100, 1000, 10000, 100000] {
        let xs = (0..n).collect::<Vec<_>>();
        benchmark_find_min(&mut g, &xs);
    }
    g.finish();

    let mut g = c.benchmark_group("find_min_random");
    g.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for n in [10, 100, 1000, 10000, 100000] {
        let xs = gen_random(n);
        benchmark_find_min(&mut g, &xs);
    }
    g.finish();

    let mut g = c.benchmark_group("delete_min_asc_sorted");
    g.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for n in [10, 100, 1000, 10000, 100000] {
        let xs = (0..n).collect::<Vec<_>>();
        benchmark_delete_min(&mut g, &xs);
    }
    g.finish();

    let mut g = c.benchmark_group("delete_min_random");
    g.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for n in [10, 100, 1000, 10000, 100000] {
        let xs = gen_random(n);
        benchmark_delete_min(&mut g, &xs);
    }
    g.finish();
}

criterion_group!(benches, benchmark_heaps);
criterion_main!(benches);
