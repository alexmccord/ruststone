use std::{
    cell::RefCell,
    collections::HashSet,
    ops::{Index, IndexMut},
};

use fnv::FnvHashMap;

use crate::{
    vec3::Vec3,
    voxels::{DustVoxel, Facing, TorchVoxel, Voxel},
    ConstraintGraph, Redstone, RedstoneArena,
};

struct Neighbors<T> {
    up: T,
    down: T,
    left: T,
    right: T,
    front: T,
    back: T,
}

impl<T> Neighbors<T> {
    fn new(up: T, down: T, left: T, right: T, front: T, back: T) -> Neighbors<T> {
        Neighbors {
            up,
            down,
            left,
            right,
            front,
            back,
        }
    }
}

struct NeighborsIter<'a, T> {
    neighbors: &'a Neighbors<T>,
    idx: u8,
}

impl<'a, T> Iterator for NeighborsIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.idx {
            0 => Some(&self.neighbors.up),
            1 => Some(&self.neighbors.down),
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

impl<'a, T> IntoIterator for &'a Neighbors<T> {
    type Item = &'a T;
    type IntoIter = NeighborsIter<'a, T>;

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
        self.voxels.get(&vec).unwrap_or_default()
    }
}

impl IndexMut<Vec3> for World<'_> {
    fn index_mut(&mut self, vec: Vec3) -> &mut Voxel {
        self.voxels
            .entry(vec)
            .or_insert_with(|| Voxel::air().voxel())
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
        for cg in self.get_constraint_graphs() {
            cg.solve_constraints();
        }
    }

    pub fn get(&'r self, vec3: Vec3) -> Option<&'r Redstone<'r>> {
        if let Some(redstone) = self.redstones.borrow().get(&vec3) {
            return Some(redstone);
        }

        let voxel = &self[vec3];
        let redstone = match voxel {
            Voxel::Air(..) => None,
            Voxel::Stone(..) => Some(self.redstone_arena.make_block(&voxel.get_name(vec3))),
            Voxel::Torch(..) => Some(self.redstone_arena.make_torch(&voxel.get_name(vec3))),
            Voxel::Dust(..) => Some(self.redstone_arena.make_dust(&voxel.get_name(vec3))),
        };

        if let Some(redstone) = redstone {
            self.redstones.borrow_mut().insert(vec3, redstone);
        }

        redstone
    }

    fn neighbors(vec3: Vec3) -> Neighbors<Vec3> {
        Neighbors::new(
            vec3.up(),
            vec3.down(),
            vec3.west(),
            vec3.east(),
            vec3.north(),
            vec3.south(),
        )
    }

    fn get_constraint_graphs(&'r self) -> Vec<ConstraintGraph<'r>> {
        for (vec3, voxel) in &self.voxels {
            match voxel {
                Voxel::Air(..) => continue,
                Voxel::Stone(_) => (), // I think this is no-op in general.
                Voxel::Torch(torch) => self.visit_torch_voxel(*vec3, torch),
                Voxel::Dust(dust) => self.visit_dust_voxel(*vec3, dust),
            }
        }

        // We need to be able to find all the disjoint redstone graphs so that we
        // know how to collect the constraints from each disjoint redstone graphs
        // in order to dispatch all of them.
        let mut seen = HashSet::new();
        let mut cgs = Vec::new();

        for &redstone in self.redstones.borrow().values() {
            if seen.contains(&(redstone as *const Redstone)) {
                continue;
            }

            for r in redstone {
                seen.insert(r as *const Redstone);
            }

            cgs.push(ConstraintGraph::collect(redstone));
        }

        cgs
    }

    fn visit_torch_voxel(&'r self, vec3: Vec3, torch: &TorchVoxel) {
        let redstone = self.get(vec3).unwrap();

        let placed_on_vec3 = match &torch.facing {
            Some(Facing::North) => vec3.south(),
            Some(Facing::East) => vec3.west(),
            Some(Facing::West) => vec3.east(),
            Some(Facing::South) => vec3.north(),
            None => vec3.down(),
        };

        for neighbor in &World::neighbors(vec3) {
            // The torch is never linked to the voxel for which it is placed upon.
            if *neighbor == placed_on_vec3 {
                continue;
            }

            let ok = match (&self[*neighbor], (*neighbor - vec3)) {
                (Voxel::Stone(_), Vec3(0, 1, 0)) => true,
                (Voxel::Dust(_), Vec3(_, 0 | -1, _)) => true,
                (_, _) => false,
            };

            if ok {
                redstone.link(self.get(*neighbor).unwrap());
            }
        }

        assert!(self[placed_on_vec3].is_stone());
        self.get(placed_on_vec3).unwrap().link(redstone);
    }

    fn is_linkable_from_dust(&self, dust: (Vec3, &DustVoxel), other: (Vec3, &Voxel)) -> bool {
        match ((dust.0 - other.0).abs(), other.1) {
            (Vec3(0, 0, 0), _) => panic!("Same voxel?"),
            (Vec3(0, 0, 1), Voxel::Dust(..) | Voxel::Stone(..)) => true,
            (Vec3(1, 0, 0), Voxel::Dust(..) | Voxel::Stone(..)) => true,
            (_, _) => false,
        }
    }

    fn visit_dust_voxel(&'r self, vec3: Vec3, dust: &DustVoxel) {
        let redstone = self.get(vec3).unwrap();
        let neighbors = World::neighbors(vec3);

        // A dust must be placed on a stone at all times.
        assert!(self[neighbors.down].is_stone());
        // redstone.link(self.get(neighbors.down).unwrap());

        for neighbor in &neighbors {
            if self.is_linkable_from_dust((vec3, dust), (*neighbor, &self[*neighbor])) {
                redstone.link(self.get(*neighbor).unwrap());
            }

            if self[neighbors.up].is_air() && self[neighbor.up()].is_dust() {
                redstone.link(self.get(neighbor.up()).unwrap());
            }
        }
    }
}
