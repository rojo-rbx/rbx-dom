/// The bitflags crate doesn't support iterating over the bits that are set in
/// the flag. In order to generate lists of flag names, we create a macro that
/// abstracts over the bitflags macro and additionally implements IntoIterator
/// on the type.
///
/// To avoid pulling in a dependency on either the `paste!` or `concat_idents!`
/// macros, the caller has to pass inthe name of the iterator type to define.
macro_rules! bitterflag {
    ($struct_name: ident + $iter_name: ident : $width: ident { $(const $const_name: ident = $const_value: expr;)* }) => {
        bitflags::bitflags! {
            #[derive(Serialize, Deserialize)]
            pub struct $struct_name: $width {
                $(const $const_name = $const_value;)*
            }
        }

        pub struct $iter_name {
            inner: Box<dyn Iterator<Item = $struct_name>>,
        }

        impl Iterator for $iter_name {
            type Item = $struct_name;

            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next()
            }
        }

        impl IntoIterator for $struct_name {
            type Item = Self;
            type IntoIter = $iter_name;

            fn into_iter(self) -> Self::IntoIter {
                static ALL_TAGS: &[$struct_name] = &[
                    $($struct_name::$const_name,)*
                ];

                $iter_name {
                    inner: Box::new(
                        ALL_TAGS
                            .iter()
                            .cloned()
                            .filter(move |flag| self.contains(*flag)),
                    ),
                }
            }
        }
    };
}
