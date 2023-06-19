use std::io;

use rbx_dom_weak::types::CFrame;

use super::{EncodeError, XmlWriter};

pub fn cframe_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &CFrame,
) -> Result<(), EncodeError> {
    writer.write_rbx("X", value.position.x)?;
    writer.write_rbx("Y", value.position.y)?;
    writer.write_rbx("Z", value.position.z)?;

    writer.write_rbx("R00", value.orientation.x.x)?;
    writer.write_rbx("R01", value.orientation.x.y)?;
    writer.write_rbx("R02", value.orientation.x.z)?;

    writer.write_rbx("R10", value.orientation.y.x)?;
    writer.write_rbx("R11", value.orientation.y.y)?;
    writer.write_rbx("R12", value.orientation.y.z)?;

    writer.write_rbx("R20", value.orientation.z.x)?;
    writer.write_rbx("R21", value.orientation.z.y)?;
    writer.write_rbx("R22", value.orientation.z.z)?;

    Ok(())
}
