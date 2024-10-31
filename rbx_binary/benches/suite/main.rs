mod util;

use crate::util::bench;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn folders_100(c: &mut Criterion) {
    bench(
        c,
        "100 Folders",
        include_bytes!("../files/folders-100.rbxm"),
    )
}

pub fn deep_folders_100(c: &mut Criterion) {
    bench(
        c,
        "100 Deep Folders",
        include_bytes!("../files/deep-folders-100.rbxm"),
    )
}

pub fn modulescripts_100_lines_100(c: &mut Criterion) {
    bench(
        c,
        "100 100-line ModuleScripts",
        include_bytes!("../files/modulescripts-100-lines-100.rbxm"),
    )
}

pub fn parts_1000(c: &mut Criterion) {
    bench(c, "1,000 Parts", include_bytes!("../files/parts-1000.rbxm"))
}

criterion_group!(
    bench_suite,
    folders_100,
    deep_folders_100,
    modulescripts_100_lines_100,
    parts_1000,
);

criterion_main!(bench_suite);
