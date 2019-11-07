use std::io::{self, Write};

use rbx_dom_weak::RbxValue;

use crate::core::RbxWriteExt;

pub trait BinaryType {
    fn write_values<W: Write, I, T>(output: W, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = T>,
        T: AsRef<RbxValue>;
}

pub struct StringType;

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
