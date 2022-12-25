use crate::vec3::Vec3;

pub enum Facing {
    North,
    East,
    West,
    South,
}

pub struct RedstoneTorch {
    /// The facing is the opposite, e.g. if this torch is placed on the east side of the block,
    /// then this facing is East, not West.
    pub facing: Option<Facing>,
}

pub enum Voxel {
    Air,
    Stone,
    Torch(RedstoneTorch),
    Dust,
}

impl Voxel {
    pub fn air() -> Voxel {
        Voxel::Air
    }

    pub fn stone() -> Voxel {
        Voxel::Stone
    }

    pub fn torch(facing: Option<Facing>) -> Voxel {
        Voxel::Torch(RedstoneTorch { facing })
    }

    pub fn dust() -> Voxel {
        Voxel::Dust
    }

    pub fn get_name(&self, vec3: Vec3) -> String {
        match self {
            Voxel::Air => format!("air {vec3}"),
            Voxel::Stone => format!("stone {vec3}"),
            Voxel::Torch(..) => format!("torch {vec3}"),
            Voxel::Dust => format!("dust {vec3}"),
        }
    }

    /// Returns `true` if the voxel is [`Air`].
    ///
    /// [`Air`]: Voxel::Air
    #[must_use]
    pub fn is_air(&self) -> bool {
        matches!(self, Self::Air)
    }

    /// Returns `true` if the voxel is [`Stone`].
    ///
    /// [`Stone`]: Voxel::Stone
    #[must_use]
    pub fn is_stone(&self) -> bool {
        matches!(self, Self::Stone)
    }

    /// Returns `true` if the voxel is [`Torch`].
    ///
    /// [`Torch`]: Voxel::Torch
    #[must_use]
    pub fn is_torch(&self) -> bool {
        matches!(self, Self::Torch(..))
    }

    /// Returns `true` if the voxel is [`Dust`].
    ///
    /// [`Dust`]: Voxel::Dust
    #[must_use]
    pub fn is_dust(&self) -> bool {
        matches!(self, Self::Dust)
    }
}
