use rbx_dom_weak::types::{ContentType, Variant};
use std::{fs::File, io::BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args_os()
        .nth(1)
        .ok_or("usage: read_archive <place.rbx>")?;
    let mut archive =
        rbx_archive::Reader::new(BufReader::new(File::open(path)?), Default::default())?;
    println!("{} content records", archive.manifest().content.len());
    let dom = archive.deserialize()?;
    for instance in dom.descendants() {
        for (name, value) in &instance.properties {
            if name.as_str() == "Source" {
                if let Variant::String(source) = value {
                    println!("{}.Source: {} bytes", instance.class, source.len());
                }
            }
            if let Variant::Content(content) = value {
                if let ContentType::Embedded(content) = content.value() {
                    let envelope = rbx_archive::ContentEnvelope::from_bytes(content.data.data())?;
                    println!(
                        "{}.{}: {} payload bytes, metadata {}",
                        instance.class,
                        name,
                        envelope.payload.len(),
                        envelope.metadata
                    );
                }
            }
        }
    }
    println!("{} file instances", dom.descendants().count() - 1);
    Ok(())
}
