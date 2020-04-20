use rbx_reflection::ReflectionDatabase;

static ENCODED_DATABASE: &[u8] = include_bytes!("../database.msgpack");

lazy_static::lazy_static! {
    static ref DATABASE: ReflectionDatabase<'static> = {
        rmp_serde::decode::from_slice(ENCODED_DATABASE).unwrap()
    };
}

pub fn get() -> &'static ReflectionDatabase<'static> {
    &DATABASE
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke_test() {
        let _database = get();
    }
}
