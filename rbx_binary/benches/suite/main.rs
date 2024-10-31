mod util;

use crate::util::bench;
use criterion::{criterion_group, criterion_main, Criterion, SamplingMode, Throughput};

pub fn folders_100(c: &mut Criterion) {
    let bytes = include_bytes!("../files/folders-100.rbxm");
    bench(
        c.benchmark_group("100 Folders")
            .throughput(Throughput::Bytes(bytes.len() as u64)),
        bytes,
    )
}

pub fn deep_folders_100(c: &mut Criterion) {
    let bytes = include_bytes!("../files/deep-folders-100.rbxm");
    bench(
        c.benchmark_group("100 Deep Folders")
            .throughput(Throughput::Bytes(bytes.len() as u64)),
        bytes,
    )
}

pub fn modulescripts_100_lines_100(c: &mut Criterion) {
    let bytes = include_bytes!("../files/modulescripts-100-lines-100.rbxm");
    bench(
        c.benchmark_group("100 100-line ModuleScripts")
            .throughput(Throughput::Bytes(bytes.len() as u64)),
        bytes,
    )
}

pub fn parts_1000(c: &mut Criterion) {
    let bytes = include_bytes!("../files/parts-1000.rbxm");
    bench(
        c.benchmark_group("1,000 Parts")
            .sampling_mode(SamplingMode::Flat)
            .throughput(Throughput::Bytes(bytes.len() as u64)),
        bytes,
    )
}

criterion_group!(
    bench_suite,
    folders_100,
    deep_folders_100,
    modulescripts_100_lines_100,
    parts_1000,
);

criterion_main!(bench_suite);
