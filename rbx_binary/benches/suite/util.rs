use criterion::{measurement::Measurement, BatchSize, BenchmarkGroup, Throughput};
use rbx_binary::{DecodeOptions, DecompressedFile};

pub(crate) fn bench<T: Measurement>(group: &mut BenchmarkGroup<T>, bench_file: &'static [u8]) {
    serialize_bench(group, bench_file);
    deserialize_bench(group, bench_file);
}

fn serialize_bench<T: Measurement>(group: &mut BenchmarkGroup<T>, buffer: &[u8]) {
    let file = DecompressedFile::from_reader(buffer).unwrap();
    let tree = file.deserialize(DecodeOptions::read_unknown()).unwrap();
    let root_ref = tree.root_ref();
    let mut buffer = Vec::new();

    rbx_binary::to_writer(&mut buffer, &tree, &[root_ref]).unwrap();
    let buffer_len = buffer.len();
    let batch_size = if buffer_len > 1024 {
        BatchSize::LargeInput
    } else {
        BatchSize::SmallInput
    };

    group
        .throughput(Throughput::Bytes(buffer_len as u64))
        .bench_function("Serialize", |b| {
            b.iter_batched(
                || Vec::with_capacity(buffer_len),
                |mut buffer: Vec<u8>| {
                    rbx_binary::to_writer(&mut buffer, &tree, &[root_ref]).unwrap();
                },
                batch_size,
            )
        });
}

fn deserialize_bench<T: Measurement>(group: &mut BenchmarkGroup<T>, buffer: &[u8]) {
    group
        .throughput(Throughput::Bytes(buffer.len() as u64))
        .bench_function("Deserialize", |bencher| {
            bencher.iter(|| {
                DecompressedFile::from_reader(buffer)
                    .unwrap()
                    .deserialize(DecodeOptions::read_unknown())
                    .unwrap();
            });
        });
}
