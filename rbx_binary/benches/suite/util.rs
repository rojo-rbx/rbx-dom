use criterion::{measurement::Measurement, BatchSize, BenchmarkGroup, Criterion};
use rbx_dom_weak::WeakDom;

pub(crate) fn bench(c: &mut Criterion, name: &str, bench_file: &'static [u8]) {
    let mut group = c.benchmark_group(name);
    let tree = rbx_binary::from_reader(bench_file).unwrap();

    serialize_bench(&mut group, &tree);
    deserialize_bench(&mut group, bench_file);
    group.finish();
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
