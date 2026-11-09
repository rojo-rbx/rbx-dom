use std::{
    convert::TryFrom,
    io::{self, Write},
};

// Fixtures use uncompressed native chunks, not a second decoder. Their two
// instances deliberately share a name and exercise repeated losses.
pub(crate) struct Model {
    pub(crate) bytes: Vec<u8>,
}

impl Model {
    pub(crate) fn new(class: &str) -> io::Result<Self> {
        let mut model = Self {
            bytes: b"<roblox!\x89\xff\r\n\x1a\n\0\0".to_vec(),
        };
        model.bytes.extend(1u32.to_le_bytes());
        model.bytes.extend(2u32.to_le_bytes());
        model.bytes.extend([0; 8]);
        let mut inst = 7u32.to_le_bytes().to_vec();
        write_string(&mut inst, class.as_bytes())?;
        inst.push(0);
        inst.extend(2u32.to_le_bytes());
        inst.extend(referents(&[10, 20]));
        model.chunk(*b"INST", &inst)?;
        Ok(model)
    }

    pub(crate) fn chunk(&mut self, name: [u8; 4], data: &[u8]) -> io::Result<()> {
        self.bytes.write_all(&name)?;
        self.bytes.extend(0u32.to_le_bytes());
        self.bytes.extend(
            u32::try_from(data.len())
                .map_err(io::Error::other)?
                .to_le_bytes(),
        );
        self.bytes.extend(0u32.to_le_bytes());
        self.bytes.write_all(data)
    }

    pub(crate) fn property(&mut self, name: &str, data: &[u8]) -> io::Result<()> {
        let mut prop = 7u32.to_le_bytes().to_vec();
        write_string(&mut prop, name.as_bytes())?;
        prop.extend(data);
        self.chunk(*b"PROP", &prop)
    }

    pub(crate) fn finish(mut self) -> io::Result<Vec<u8>> {
        let mut parents = vec![0];
        parents.extend(2u32.to_le_bytes());
        parents.extend(referents(&[10, 20]));
        parents.extend(referents(&[-1, -1]));
        self.chunk(*b"PRNT", &parents)?;
        self.chunk(*b"END\0", b"</roblox>")?;
        Ok(self.bytes)
    }
}

fn write_string(bytes: &mut Vec<u8>, value: &[u8]) -> io::Result<()> {
    bytes.extend(
        u32::try_from(value.len())
            .map_err(io::Error::other)?
            .to_le_bytes(),
    );
    bytes.write_all(value)
}

pub(crate) fn interleaved(values: &[i32]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for byte in 0..4 {
        for value in values {
            let transformed = (value << 1) ^ (value >> 31);
            bytes.push(transformed.to_be_bytes()[byte]);
        }
    }
    bytes
}

pub(crate) fn referents(values: &[i32]) -> Vec<u8> {
    let mut previous = 0;
    let deltas: Vec<_> = values
        .iter()
        .map(|value| {
            let delta = value - previous;
            previous = *value;
            delta
        })
        .collect();
    interleaved(&deltas)
}

pub(crate) fn strings(first: &[u8], second: &[u8]) -> io::Result<Vec<u8>> {
    let mut bytes = vec![1];
    write_string(&mut bytes, first)?;
    write_string(&mut bytes, second)?;
    Ok(bytes)
}
