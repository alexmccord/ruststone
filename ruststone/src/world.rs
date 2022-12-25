use std::{
    cell::RefCell,
    collections::VecDeque,
    ops::{Index, IndexMut},
};

use fnv::FnvHashMap;

use crate::{vec3::Vec3, voxels::Voxel, ConstraintGraph, Redstone, RedstoneArena};

struct Neighbors<T> {
    top: T,
    bottom: T,
    left: T,
    right: T,
    front: T,
    back: T,
}

impl<T> Neighbors<T> {
    fn new(up: T, down: T, left: T, right: T, front: T, back: T) -> Neighbors<T> {
        Neighbors {
            top: up,
            bottom: down,
            left,
            right,
            front,
            back,
        }
    }

    fn map<F: FnMut(&T) -> U, U>(&self, mut f: F) -> Neighbors<U> {
        Neighbors::new(
            f(&self.top),
            f(&self.bottom),
            f(&self.left),
            f(&self.right),
            f(&self.front),
            f(&self.back),
        )
    }
}

struct NeighborsIter<'n, T> {
    neighbors: &'n Neighbors<T>,
    idx: u8,
}

impl<'n, T> Iterator for NeighborsIter<'n, T> {
    type Item = &'n T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.idx {
            0 => Some(&self.neighbors.top),
            1 => Some(&self.neighbors.bottom),
            2 => Some(&self.neighbors.left),
            3 => Some(&self.neighbors.right),
            4 => Some(&self.neighbors.front),
            5 => Some(&self.neighbors.back),
            _ => None,
        };
        self.idx += 1;
        result
    }
}

impl<'n, T> IntoIterator for &'n Neighbors<T> {
    type Item = &'n T;
    type IntoIter = NeighborsIter<'n, T>;

    fn into_iter(self) -> Self::IntoIter {
        NeighborsIter {
            neighbors: self,
            idx: 0,
        }
    }
}

#[derive(Default)]
pub struct World<'r> {
    redstone_arena: RedstoneArena<'r>,
    voxels: FnvHashMap<Vec3, Voxel>,
    redstones: RefCell<FnvHashMap<Vec3, &'r Redstone<'r>>>,
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
            redstones: RefCell::new(FnvHashMap::default()),
        }
    }

    pub fn run(&'r self) {
        let cg = self.to_constraint_graph();
        cg.solve_constraints();
    }

    pub fn get_redstone_at(&'r self, vec3: Vec3) -> Option<&'r Redstone<'r>> {
        if let Some(redstone) = self.redstones.borrow().get(&vec3) {
            return Some(redstone);
        }

        let voxel = &self[vec3];
        let redstone = match voxel {
            Voxel::Air => None,
            Voxel::Stone => Some(self.redstone_arena.make_block(&voxel.get_name(vec3))),
            Voxel::Torch(..) => Some(self.redstone_arena.make_torch(&voxel.get_name(vec3))),
            Voxel::Dust => Some(self.redstone_arena.make_dust(&voxel.get_name(vec3))),
        };

        if let Some(redstone) = redstone {
            self.redstones.borrow_mut().insert(vec3, redstone);
        }

        redstone
    }

    fn neighboring_vec3s(vec3: Vec3) -> Neighbors<Vec3> {
        Neighbors::new(
            vec3.up(),
            vec3.down(),
            vec3.left(),
            vec3.right(),
            vec3.front(),
            vec3.back(),
        )
    }

    fn neighboring_voxels(&self, vec3: Vec3) -> Neighbors<&Voxel> {
        World::neighboring_vec3s(vec3).map(|v| &self[*v])
    }

    fn neighboring_redstones(&'r self, vec3: Vec3) -> Neighbors<Option<&'r Redstone<'r>>> {
        World::neighboring_vec3s(vec3).map(|v| self.get_redstone_at(*v))
    }

    fn neighbors(&'r self, vec3: Vec3) -> Neighbors<(Vec3, &Voxel, Option<&'r Redstone<'r>>)> {
        World::neighboring_vec3s(vec3).map(|v| (*v, &self[*v], self.get_redstone_at(*v)))
    }

    pub fn to_constraint_graph(&'r self) -> ConstraintGraph<'r> {
        let mut queue = VecDeque::new();
        queue.extend(self.voxels.iter());

        // TODO: We need to compute a weighted edge for each redstone dust, so we do this in a separate queue.
        // let mut redstone_dusts = VecDeque::new();

        while let Some((vec3, voxel)) = queue.pop_back() {
            match voxel {
                Voxel::Air => continue,
                Voxel::Stone => self.visit_stone_voxel(*vec3),
                Voxel::Torch(facing) => self.visit_torch_voxel(*vec3),
                Voxel::Dust => self.visit_dust_voxel(*vec3),
            }
        }

        // I hope it's okay to do this. I think it is, because even though the constraint set is ordered
        // and we're about to give `ConstraintGraph::collect` the first arbitrary node from an unordered
        // container, the solver should always solve them in the right order regardless.
        ConstraintGraph::collect(self.redstones.borrow().values().next().unwrap())
    }

    fn visit_stone_voxel(&'r self, vec3: Vec3) {
        assert!(self[vec3].is_stone());
    }

    fn visit_torch_voxel(&'r self, vec3: Vec3) {
        assert!(self[vec3].is_torch());
    }

    fn visit_dust_voxel(&'r self, vec3: Vec3) {
        assert!(self[vec3].is_dust());

        let redstone = self.get_redstone_at(vec3).unwrap();
        let neighbors = self.neighbors(vec3);

        // A dust must be placed on a stone at all times.
        assert!(neighbors.bottom.1.is_stone());

        // We need to accurately reflect the same behavior that Redstone dusts have in Minecraft,
        // where a dust in a straight line does not include some redstone nodes as its neighbor,
        // except on the ends or below.
        //
        // This rule is different for other redstone nodes that causes a direct connection, e.g.
        // in the same circumstance, a redstone repeater on the side of the straight line is a viable
        // neighbor.
    }
}
