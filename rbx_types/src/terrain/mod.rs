// Terrain has two binary formats, being the material colors and smooth grid blobs.
// The smooth grid spec can be found here: https://github.com/rojo-rbx/rbx-dom/blob/terrain/docs/smooth-grid.md
// The material colors spec can be found here: https://github.com/rojo-rbx/rbx-dom/blob/master/docs/binary-strings.md#materialcolors

mod material_colors;
mod smooth_grid;

pub use self::material_colors::*;
pub use self::smooth_grid::*;

use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TerrainMaterials {
    Air,
    Water,
    Grass,
    Slate,
    Concrete,
    Brick,
    Sand,
    WoodPlanks,
    Rock,
    Glacier,
    Snow,
    Sandstone,
    Mud,
    Basalt,
    Ground,
    CrackedLava,
    Asphalt,
    Cobblestone,
    Ice,
    LeafyGrass,
    Salt,
    Limestone,
    Pavement,
}

impl fmt::Display for TerrainMaterials {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
