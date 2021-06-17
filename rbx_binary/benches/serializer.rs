use criterion::{criterion_group, criterion_main, Criterion};

use rbx_dom_weak::{InstanceBuilder, WeakDom};

pub fn ser_folders_100(c: &mut Criterion) {
    let mut tree = WeakDom::new(InstanceBuilder::new("Folder").with_name("Container"));
    let root_ref = tree.root_ref();

    for i in 0..99 {
        tree.insert(
            root_ref,
            InstanceBuilder::new("Folder").with_name(format!("Folder {}", i)),
        );
    }

    let mut buffer = Vec::new();

    // Encode once into the buffer to pre-size it.
    rbx_binary::to_writer(&mut buffer, &tree, &[root_ref]).unwrap();
    buffer.clear();

    c.bench_function("Serialize 100 Folders", |b| {
        b.iter(|| {
            rbx_binary::to_writer(&mut buffer, &tree, &[root_ref]).unwrap();
            buffer.clear();
        });
    });
}

criterion_group!(serializer, ser_folders_100);
criterion_main!(serializer);
