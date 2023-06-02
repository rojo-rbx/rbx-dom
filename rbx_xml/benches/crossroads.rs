use criterion::{criterion_group, criterion_main, Criterion};
use rbx_dom_weak::{types::Ref, WeakDom};

static BUFFER: &[u8] = include_bytes!("crossroads.rbxlx");

pub fn de_crossroads(c: &mut Criterion) {
    c.bench_function("Deserialize crossroads", |b| {
        b.iter(|| {
            rbx_xml::from_reader_default(BUFFER).unwrap();
        });
    });
}

fn ser_crossroads(c: &mut Criterion) {
    let (referents, dom) = {
        let (root, instances) = rbx_xml::from_reader_default(BUFFER).unwrap().into_raw();
        let referents: Vec<Ref> = instances.keys().copied().collect();
        (referents, WeakDom::from_raw(root, instances))
    };
    let mut buffer = Vec::new();
    rbx_xml::to_writer_default(&mut buffer, &dom, &referents).unwrap();
    buffer.clear();

    c.bench_function("Serialize crossroads", |b| {
        b.iter(|| {
            rbx_xml::to_writer_default(&mut buffer, &dom, &referents).unwrap();
            buffer.clear();
        })
    });
}

criterion_group!(crossroads, de_crossroads, ser_crossroads);
criterion_main!(crossroads);
