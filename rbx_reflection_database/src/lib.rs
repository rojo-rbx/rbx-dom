use rbx_reflection::ReflectionDatabase;

static ENCODED_DATABASE: &[u8] = include_bytes!("../database.msgpack");

lazy_static::lazy_static! {
    static ref DATABASE: ReflectionDatabase<'static> = {
        rmp_serde::decode::from_slice(ENCODED_DATABASE).unwrap_or_else(|e| panic!("could not decode reflection database because: {}", e))
    };
}

pub fn get() -> &'static ReflectionDatabase<'static> {
    &DATABASE
}

#[cfg(test)]
mod test {
    use rbx_reflection::ClassDescriptor;

    use super::*;

    #[test]
    fn smoke_test() {
        let _database = get();
    }

    #[test]
    fn superclasses_iter_test() {
        let database = get();
        let part_class_descriptor = database.classes.get("Part");
        let mut iter = database.superclasses_iter(part_class_descriptor.unwrap());
        fn class_descriptor_eq(lhs: Option<&ClassDescriptor>, rhs: Option<&ClassDescriptor>) {
            let eq = match (lhs, rhs) {
                (Some(lhs), Some(rhs)) => lhs.name == rhs.name,
                (None, None) => true,
                _ => false,
            };
            assert!(eq, "{:?} != {:?}", lhs, rhs);
        }
        class_descriptor_eq(iter.next(), part_class_descriptor);
        class_descriptor_eq(iter.next(), database.classes.get("FormFactorPart"));
        class_descriptor_eq(iter.next(), database.classes.get("BasePart"));
        class_descriptor_eq(iter.next(), database.classes.get("PVInstance"));
        class_descriptor_eq(iter.next(), database.classes.get("Instance"));
        class_descriptor_eq(iter.next(), None);
    }
}
