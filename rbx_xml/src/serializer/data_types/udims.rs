use std::io;

use super::{EncodeError, XmlWriter};
use rbx_dom_weak::types::{UDim, UDim2};

pub fn udim_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &UDim,
) -> Result<(), EncodeError> {
    writer.write_rbx("S", value.scale)?;
    writer.write_rbx("O", value.offset)?;
    Ok(())
}

pub fn udim2_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &UDim2,
) -> Result<(), EncodeError> {
    writer.write_rbx("XS", value.x.scale)?;
    writer.write_rbx("XO", value.x.offset)?;
    writer.write_rbx("YS", value.y.scale)?;
    writer.write_rbx("YO", value.y.offset)?;
    Ok(())
}
