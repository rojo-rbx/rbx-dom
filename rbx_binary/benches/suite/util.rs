use criterion::{measurement::Measurement, BatchSize, BenchmarkGroup};
use rbx_dom_weak::WeakDom;

pub(crate) fn bench<T: Measurement>(group: &mut BenchmarkGroup<T>, bench_file: &'static [u8]) {
    let tree = rbx_binary::from_reader(bench_file).unwrap();
    serialize_bench(group, &tree);
    deserialize_bench(group, bench_file);
}

fn serialize_bench<T: Measurement>(group: &mut BenchmarkGroup<T>, tree: &WeakDom) {
    let root_ref = tree.root_ref();
    let mut buffer = Vec::new();

    rbx_binary::to_writer(&mut buffer, tree, &[root_ref]).unwrap();
    let buffer_len = buffer.len();

    group.bench_function("Serialize", |b| {
        b.iter_batched(
            || Vec::with_capacity(buffer_len),
            |mut buffer: Vec<u8>| {
                rbx_binary::to_writer(&mut buffer, tree, &[root_ref]).unwrap();
            },
            BatchSize::SmallInput,
        )
    });
}

fn deserialize_bench<T: Measurement>(group: &mut BenchmarkGroup<T>, buffer: &[u8]) {
    group.bench_function("Deserialize", |bencher| {
        bencher.iter(|| {
            rbx_binary::from_reader(buffer).unwrap();
        });
    });
}
