pub enum CardinalDirection {
    North,
    East,
    West,
    South,
}

pub struct RedstoneTorch {
    pub facing: Option<CardinalDirection>,
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

    pub fn torch(facing: Option<CardinalDirection>) -> Voxel {
        Voxel::Torch(RedstoneTorch { facing })
    }

    pub fn dust() -> Voxel {
        Voxel::Dust
    }
}
