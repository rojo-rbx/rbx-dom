use criterion::{criterion_group, criterion_main, Criterion};

use rbx_dom_weak::{RbxInstanceProperties, RbxTree};

pub fn ser_folders_100(c: &mut Criterion) {
    let mut tree = RbxTree::new(RbxInstanceProperties {
        name: "Container".to_owned(),
        class_name: "Folder".to_owned(),
        properties: Default::default(),
    });
    let root_id = tree.get_root_id();

    for i in 0..99 {
        tree.insert_instance(
            RbxInstanceProperties {
                name: format!("Folder {}", i),
                class_name: "Folder".to_owned(),
                properties: Default::default(),
            },
            root_id,
        );
    }

    let mut buffer = Vec::new();

    // Encode once into the buffer to pre-size it.
    rbx_binary::encode(&tree, &[root_id], &mut buffer).unwrap();
    buffer.clear();

    c.bench_function("Serialize 100 Folders", |b| {
        b.iter(|| {
            rbx_binary::encode(&tree, &[root_id], &mut buffer).unwrap();
            buffer.clear();
        });
    });
}

criterion_group!(serializer, ser_folders_100);
criterion_main!(serializer);
