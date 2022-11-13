use std::ops::{Index, IndexMut};

use fnv::FnvHashMap;

use crate::{vec3::Vec3, voxels::Voxel, Redstone, RedstoneArena};

#[derive(Default)]
pub struct World<'r> {
    redstone_arena: RedstoneArena<'r>,
    voxels: FnvHashMap<Vec3, Voxel>,
}

impl Index<Vec3> for World<'_> {
    type Output = Voxel;

    fn index(&self, vec: Vec3) -> &Voxel {
        self.voxels.get(&vec).unwrap_or(&Voxel::Air)
    }
}

impl IndexMut<Vec3> for World<'_> {
    fn index_mut(&mut self, vec: Vec3) -> &mut Voxel {
        self.voxels.entry(vec).or_insert(Voxel::Air)
    }
}

impl<'r> World<'r> {
    pub fn new() -> World<'r> {
        World {
            redstone_arena: RedstoneArena::new(),
            voxels: FnvHashMap::default(),
        }
    }

    fn convert_voxel_into_redstone(&'r self, voxel: &Voxel) -> Option<&'r Redstone<'r>> {
        match voxel {
            Voxel::Air => None,
            Voxel::Stone => Some(self.redstone_arena.make_block("stone")),
            Voxel::Torch(..) => Some(self.redstone_arena.make_torch("torch")),
            Voxel::Dust => Some(self.redstone_arena.make_dust("dust")),
        }
    }

    fn neighbors(&self, vec: Vec3) -> Vec<&Voxel> {
        vec![
            &self[vec.up()],
            &self[vec.down()],
            &self[vec.left()],
            &self[vec.right()],
            &self[vec.front()],
            &self[vec.back()],
        ]
    }

    // pub fn to_redstone_graph(&self)
}
