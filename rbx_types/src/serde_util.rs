macro_rules! serde_tuple {
    (
        $(
            $type: ident (
                $( $field_name: ident : $field_type: ty ),*
            ),
        )*
    ) => {
        $(
            impl serde::Serialize for $type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    ( $( (&self).$field_name, )* ).serialize(serializer)
                }
            }

            impl<'de> serde::Deserialize<'de> for $type {
                fn deserialize<D>(deserializer: D) -> Result<$type, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    serde::Deserialize::deserialize(deserializer)
                        .map(|($( $field_name, )*)| $type {
                            $( $field_name, )*
                        })
                }
            }
        )*
    };
}
