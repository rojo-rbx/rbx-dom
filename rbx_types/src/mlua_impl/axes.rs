use mlua::{prelude::*, Variadic};

use std::fmt;

use crate::{lister::Lister, Axes};

impl Axes {
    /// Creates a new [`Axes`] using list of axes and/or faces. NormalIds
    /// (faces) are converted to the corresponding axes.
    ///
    /// ## See Also
    /// * [Axes.new on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#new)
    #[doc(alias = "new")]
    #[must_use]
    pub fn new_lua(_lua: &Lua, args: Variadic<LuaValue>) -> LuaResult<Self> {}

    /// Whether the X axis, left face, and right face are enabled/included.
    ///
    /// ## See Also
    /// * [Axes.X on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#X)
    /// * [Axes.Left on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#Left)
    /// * [Axes.Right on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#Right)
    #[doc(alias("X", "Left", "Right"))]
    #[must_use]
    pub const fn x(self) -> bool {
        self.contains(Self::X)
    }

    /// Whether the Y axis, top face, and bottom face are enabled/included.
    ///
    /// ## See Also
    /// * [Axes.Y on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#Y)
    /// * [Axes.Top on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#Top)
    /// * [Axes.Bottom on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#Bottom)
    #[doc(alias("Y", "Top", "Bottom"))]
    #[must_use]
    pub const fn y(self) -> bool {
        self.contains(Self::Y)
    }

    /// Whether the Z axis, back face, and front face are enabled/included.
    ///
    /// ## See Also
    /// * [Axes.Z on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#Z)
    /// * [Axes.Back on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#Back)
    /// * [Axes.Front on Roblox Developer Hub](https://developer.roblox.com/en-us/api-reference/datatype/Axes#Front)
    #[doc(alias("Z", "Back", "Front"))]
    #[must_use]
    pub const fn z(self) -> bool {
        self.contains(Self::Z)
    }
}

impl fmt::Display for Axes {
    // This is not correct right now, because the `Lister` utility is not
    // working as intended, but if PR #491 gets merged, this will then
    // be accurate to the Roblox `tostring()`.
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let mut list = Lister::new();

        if self.contains(Self::X) {
            list.write(out, "X")?;
        }

        if self.contains(Self::Y) {
            list.write(out, "Y")?;
        }

        if self.contains(Self::Z) {
            list.write(out, "Z")?;
        }

        Ok(())
    }
}

impl LuaUserData for Axes {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("X", |_, axes| Ok(axes.x()));
        fields.add_field_method_get("Y", |_, axes| Ok(axes.y()));
        fields.add_field_method_get("Z", |_, axes| Ok(axes.z()));
        fields.add_field_method_get("Top", |_, axes| Ok(axes.y()));
        fields.add_field_method_get("Bottom", |_, axes| Ok(axes.y()));
        fields.add_field_method_get("Left", |_, axes| Ok(axes.x()));
        fields.add_field_method_get("Right", |_, axes| Ok(axes.x()));
        fields.add_field_method_get("Back", |_, axes| Ok(axes.z()));
        fields.add_field_method_get("Front", |_, axes| Ok(axes.z()));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method(LuaMetaMethod::Eq, |_, axes, other: LuaUserDataRef<Axes>| {
            Ok(*axes == *other)
        });
        methods.add_meta_method(LuaMetaMethod::ToString, |_, axes, ()| Ok(axes.to_string()));
    }
}
