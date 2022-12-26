use crate::vec3::Vec3;

pub enum Facing {
    North,
    East,
    West,
    South,
}

pub struct AirVoxel;

impl AirVoxel {
    pub fn voxel(self) -> Voxel {
        Voxel::Air(self)
    }
}

pub struct StoneVoxel;

impl StoneVoxel {
    pub fn voxel(self) -> Voxel {
        Voxel::Stone(self)
    }
}

pub struct TorchVoxel {
    /// The facing is the opposite, e.g. if this torch is placed on the east side of the block,
    /// then this facing is East, not West.
    pub(crate) facing: Option<Facing>,
}

impl TorchVoxel {
    pub fn voxel(self) -> Voxel {
        Voxel::Torch(self)
    }

    fn set_facing(mut self, facing: Facing) -> TorchVoxel {
        self.facing = Some(facing);
        self
    }

    pub fn facing_north(self) -> TorchVoxel {
        self.set_facing(Facing::North)
    }

    pub fn facing_east(self) -> TorchVoxel {
        self.set_facing(Facing::East)
    }

    pub fn facing_west(self) -> TorchVoxel {
        self.set_facing(Facing::West)
    }

    pub fn facing_south(self) -> TorchVoxel {
        self.set_facing(Facing::South)
    }
}

pub struct DustVoxel;

impl DustVoxel {
    pub fn voxel(self) -> Voxel {
        Voxel::Dust(self)
    }
}

pub enum Voxel {
    Air(AirVoxel),
    Stone(StoneVoxel),
    Torch(TorchVoxel),
    Dust(DustVoxel),
}

impl Voxel {
    pub fn air() -> AirVoxel {
        AirVoxel
    }

    pub fn stone() -> StoneVoxel {
        StoneVoxel
    }

    pub fn torch() -> TorchVoxel {
        TorchVoxel { facing: None }
    }

    pub fn dust() -> DustVoxel {
        DustVoxel
    }

    pub fn get_name(&self, vec3: Vec3) -> String {
        match self {
            Voxel::Air(..) => format!("air {vec3}"),
            Voxel::Stone(..) => format!("stone {vec3}"),
            Voxel::Torch(..) => format!("torch {vec3}"),
            Voxel::Dust(..) => format!("dust {vec3}"),
        }
    }

    /// Returns `true` if the voxel is [`Air`].
    ///
    /// [`Air`]: Voxel::Air
    #[must_use]
    pub fn is_air(&self) -> bool {
        matches!(self, Self::Air(..))
    }

    /// Returns `true` if the voxel is [`Stone`].
    ///
    /// [`Stone`]: Voxel::Stone
    #[must_use]
    pub fn is_stone(&self) -> bool {
        matches!(self, Self::Stone(..))
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
        matches!(self, Self::Dust(..))
    }
}

impl Default for &Voxel {
    fn default() -> Self {
        &Voxel::Air(AirVoxel)
    }
}
