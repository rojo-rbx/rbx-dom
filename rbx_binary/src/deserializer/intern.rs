/// A trait representing string internment that lives for at least 'output.
/// There is a blanket implementation for any function fn(&str)->&str
/// so this can be a closure operating your very own string interner.
pub trait StringIntern<'input, 'output> {
    /// Take an input string with lifetime 'input and guarantee
    /// that the identical output string lives for at least 'output.
    fn intern(&mut self, str: &'input str) -> &'output str;
}

/// A type alias for a function which implements the StringIntern trait.
pub type InternFunction<'input, 'output> = fn(&'input str) -> &'output str;

impl<'input, 'output, F> StringIntern<'input, 'output> for F
where
    F: FnMut(&'input str) -> &'output str,
{
    fn intern(&mut self, str: &'input str) -> &'output str {
        self(str)
    }
}
