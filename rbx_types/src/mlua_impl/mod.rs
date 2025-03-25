use mlua::prelude::*;

mod axes;

mod util_lister;

pub trait Global: Sized {
    const GLOBAL_NAME: &'static str;

    fn create_global(lua: &Lua) -> LuaResult<LuaValue>;
}
