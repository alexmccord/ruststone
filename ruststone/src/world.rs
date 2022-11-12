use std::ops::{Index, IndexMut};

use fnv::FnvHashMap;

use crate::{blocks::Block, vec3::Vec3};

#[derive(Default)]
pub struct World {
    voxels: FnvHashMap<Vec3, Block>,
}

impl World {
    pub fn new() -> World {
        World {
            voxels: FnvHashMap::default(),
        }
    }
}

impl Index<Vec3> for World {
    type Output = Block;

    fn index(&self, vec: Vec3) -> &Block {
        self.voxels.get(&vec).unwrap_or(&Block::Air)
    }
}

impl IndexMut<Vec3> for World {
    fn index_mut(&mut self, vec: Vec3) -> &mut Block {
        self.voxels.entry(vec).or_insert(Block::Air)
    }
}
