use std::ops::BitAnd;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use roaring::RoaringBitmap;

static N: [u32; 5] = [10, 100, 1_000, 100_000, 1_000_000];

pub fn bench_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_elements_sequential");
    for &batch_size in &N {
        group.throughput(Throughput::Elements(batch_size as u64));
        group.bench_with_input(
            BenchmarkId::new("croaring", batch_size),
            &batch_size,
            |b, &batch_size| {
                let mut bm = croaring::Bitmap::create();
                b.iter(|| {
                    for i in 0..batch_size {
                        bm.add(i as u32);
                    }
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("roaring", batch_size),
            &batch_size,
            |b, &batch_size| {
                let mut bm = RoaringBitmap::new();
                b.iter(|| {
                    for i in 0..batch_size {
                        bm.insert(i as u32);
                    }
                });
            },
        );
    }
    group.finish();
}

pub fn bench_add_shuffled(c: &mut Criterion) {
    use rand::prelude::SliceRandom;
    let mut rng = rand::thread_rng();

    let mut group = c.benchmark_group("add_elements_shuffled");
    for &batch_size in &N {
        // Shuffle insert order
        let mut shuffled = (0..batch_size).collect::<Vec<u32>>();
        shuffled.shuffle(&mut rng);

        group.throughput(Throughput::Elements(batch_size as u64));
        group.bench_with_input(
            BenchmarkId::new("croaring", batch_size),
            &batch_size,
            |b, &_batch_size| {
                let mut bm = croaring::Bitmap::create();
                b.iter(|| {
                    for i in &shuffled {
                        bm.add(*i);
                    }
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("roaring", batch_size),
            &batch_size,
            |b, &_batch_size| {
                let mut bm = RoaringBitmap::new();
                b.iter(|| {
                    for i in &shuffled {
                        bm.insert(*i);
                    }
                });
            },
        );
    }
    group.finish();
}

pub fn bench_add_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_range");
    for &batch_size in &N {
        group.throughput(Throughput::Elements(batch_size as u64));
        group.bench_with_input(
            BenchmarkId::new("croaring", batch_size),
            &batch_size,
            |b, &batch_size| {
                let mut bm = croaring::Bitmap::create();
                b.iter(|| {
                    bm.add_range(0..batch_size);
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("roaring", batch_size),
            &batch_size,
            |b, &batch_size| {
                let mut bm = RoaringBitmap::new();
                b.iter(|| {
                    bm.insert_range(0..batch_size);
                });
            },
        );
    }
    group.finish();
}

pub fn bench_collect_uint(c: &mut Criterion) {
    let mut group = c.benchmark_group("collect_uint");
    for &batch_size in &N {
        group.throughput(Throughput::Elements(batch_size as u64));
        group.bench_with_input(
            BenchmarkId::new("croaring", batch_size),
            &batch_size,
            |b, &batch_size| {
                let mut bm = croaring::Bitmap::create();
                bm.add_range(0..batch_size);
                b.iter(|| {
                    let _: Vec<u32> = bm.iter().collect();
                });
            },
        );
        group.bench_with_input(
            BenchmarkId::new("roaring", batch_size),
            &batch_size,
            |b, &batch_size| {
                let mut bm = RoaringBitmap::new();
                bm.insert_range(0..batch_size);
                b.iter(|| {
                    let _: Vec<u32> = bm.iter().collect();
                });
            },
        );
    }
    group.finish();
}

/// Benchmark performing a set union of two sets, both of size "batch_size / 2".
pub fn bench_union(c: &mut Criterion) {
    let mut group = c.benchmark_group("union");
    for &batch_size in &N {
        group.throughput(Throughput::Elements(batch_size as u64));
        group.bench_with_input(
            BenchmarkId::new("croaring", batch_size),
            &batch_size,
            |b, &batch_size| {
                let (set_a, set_b): (croaring::Bitmap, croaring::Bitmap) =
                    (0..batch_size).partition(|v| (v % 2) == 0);
                b.iter(|| black_box(set_a.and(&set_b)));
            },
        );
        group.bench_with_input(
            BenchmarkId::new("roaring", batch_size),
            &batch_size,
            |b, &batch_size| {
                let (set_a, set_b): (RoaringBitmap, RoaringBitmap) =
                    (0..batch_size).partition(|v| (v % 2) == 0);

                b.iter(|| black_box((&set_a).bitand(&set_b)));
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_add,
    bench_add_range,
    bench_add_shuffled,
    bench_collect_uint,
    bench_union,
);
criterion_main!(benches);
