use std::fmt;

/// Small utility to write lists of things.
pub fn write_comma_separated<D, I>(f: &mut fmt::Formatter, iter: I) -> fmt::Result
where
    D: fmt::Display,
    I: IntoIterator<Item = D>,
{
    let mut it = iter.into_iter();
    if let Some(first) = it.next() {
        write!(f, "{first}")?;
        for item in it {
            write!(f, ", {item}")?;
        }
    }
    Ok(())
}
