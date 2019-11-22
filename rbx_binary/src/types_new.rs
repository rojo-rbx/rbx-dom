use std::io::{self, Read, Write};

use rbx_dom_weak::RbxValue;

use crate::core::{RbxReadExt, RbxWriteExt};

pub trait BinaryType {
    fn write_values<W: Write, I, T>(output: W, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<RbxValue>;
}

pub struct StringType;

impl StringType {
    fn read_values<R: Read>(
        mut input: R,
        count: usize,
    ) -> io::Result<impl Iterator<Item = io::Result<Vec<u8>>>> {
        Ok(StringTypeIter {
            input,
            remaining: count,
        })
    }
}

struct StringTypeIter<R> {
    input: R,
    remaining: usize,
}

impl<R: Read> Iterator for StringTypeIter<R> {
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        match self.input.read_string() {
            Ok(value) => {
                self.remaining -= 1;
                Some(Ok(value.into_bytes()))
            }
            Err(err) => Some(Err(err)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl BinaryType for StringType {
    fn write_values<W: Write, I, T>(mut output: W, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<RbxValue>,
    {
        for rbx_value in values {
            let rbx_value = rbx_value.as_ref();

            match rbx_value {
                RbxValue::String { value } => {
                    output.write_string(&value)?;
                }
                _ => unimplemented!(), // TODO: error?
            }
        }

        Ok(())
    }
}

pub struct BoolType;

impl BinaryType for BoolType {
    fn write_values<W: Write, I, T>(mut output: W, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<RbxValue>,
    {
        for rbx_value in values {
            let rbx_value = rbx_value.as_ref();

            match rbx_value {
                RbxValue::Bool { value } => {
                    output.write_bool(*value)?;
                }
                _ => unimplemented!(), // TODO: error?
            }
        }

        Ok(())
    }
}
