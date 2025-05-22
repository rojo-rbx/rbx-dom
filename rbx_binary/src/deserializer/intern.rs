/// A trait representing string internment that lives for at least 'long.
/// There is a blanket implementation for any function fn(&str)->&str
/// so this can be a closure operating your very own string interner.
pub trait StringIntern<'short, 'long> {
    /// Take an input string with lifetime 'short and guarantee
    /// that the identical output string lives for at least 'long.
    fn intern(&mut self, str: &'short str) -> &'long str;
}

/// A type alias for a function which implements the StringIntern trait.
pub type InternFunction<'short, 'long> = fn(&'short str) -> &'long str;

impl<'short, 'long, F> StringIntern<'short, 'long> for F
where
    F: FnMut(&'short str) -> &'long str,
{
    fn intern(&mut self, str: &'short str) -> &'long str {
        self(str)
    }
}
