use std::fmt;

/// Small utility to write lists of things.
pub fn write_comma_separated<D, I, F>(f: &mut fmt::Formatter, iter: I, mut writer: F) -> fmt::Result
where
    D: fmt::Display,
    I: IntoIterator<Item = D>,
    F: FnMut(&mut fmt::Formatter, D) -> fmt::Result,
{
    let mut it = iter.into_iter();
    if let Some(first) = it.next() {
        writer(f, first)?;
        for item in it {
            write!(f, ", ")?;
            writer(f, item)?;
        }
    }
    Ok(())
}
