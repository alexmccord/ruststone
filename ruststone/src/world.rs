use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    ops::{Index, IndexMut},
};

use fnv::FnvHashMap;

use crate::{
    add_weighted_edge,
    vec3::Vec3,
    voxels::{DustVoxel, Facing, TorchVoxel, Voxel},
    ConstraintGraph, Redstone, RedstoneArena,
};

#[derive(Clone, Copy)]
struct VoxelCtxt<'r> {
    vec3: Vec3,
    voxel: &'r Voxel,
    redstone: Option<&'r Redstone<'r>>,
}

struct Neighbors<T: Copy> {
    components: [T; 6],
}

impl<T: Copy> Neighbors<T> {
    fn new(up: T, down: T, left: T, right: T, front: T, back: T) -> Neighbors<T> {
        Neighbors::from_components([up, down, left, right, front, back])
    }

    fn from_components(components: [T; 6]) -> Neighbors<T> {
        Neighbors { components }
    }

    fn map<F: FnMut(T) -> U, U: Copy>(&self, f: F) -> Neighbors<U> {
        Neighbors::from_components(self.components.map(f))
    }

    fn up(&self) -> &T {
        &self.components[0]
    }

    fn down(&self) -> &T {
        &self.components[1]
    }

    fn left(&self) -> &T {
        &self.components[2]
    }

    fn right(&self) -> &T {
        &self.components[3]
    }

    fn front(&self) -> &T {
        &self.components[4]
    }

    fn back(&self) -> &T {
        &self.components[5]
    }
}

struct NeighborsIter<'n, T: Copy> {
    neighbors: &'n Neighbors<T>,
    idx: u8,
}

impl<'n, T: Copy> Iterator for NeighborsIter<'n, T> {
    type Item = &'n T;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.idx;
        self.idx += 1;
        self.neighbors.components.get(i as usize)
    }
}

impl<'n, T: Copy> IntoIterator for &'n Neighbors<T> {
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

    fn vec3_neighbors(vec3: Vec3) -> Neighbors<Vec3> {
        Neighbors::new(
            vec3.up(),
            vec3.down(),
            vec3.west(),
            vec3.east(),
            vec3.north(),
            vec3.south(),
        )
    }

    fn neighbors(&'r self, vec3: Vec3) -> Neighbors<VoxelCtxt> {
        World::vec3_neighbors(vec3).map(|v| VoxelCtxt {
            vec3: v,
            voxel: &self[v],
            redstone: self.get(v),
        })
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

        for (vec3, voxel) in &self.voxels {
            if !voxel.is_dust() {
                continue;
            }

            let dust = self.get(*vec3).unwrap();
            let mut queue = VecDeque::new();
            queue.push_front((0, *vec3));

            let mut anticycle = HashSet::new();

            while let Some(current) = queue.pop_front() {
                if anticycle.contains(&current.1) {
                    continue;
                }

                anticycle.insert(current.1);

                let voxel = &self[current.1];
                let source = self.get(current.1);

                match voxel {
                    Voxel::Air(_) => continue,
                    Voxel::Stone(_) => add_weighted_edge(dust, source.unwrap(), current.0),
                    Voxel::Torch(_) => add_weighted_edge(dust, source.unwrap(), current.0),
                    Voxel::Dust(_) => {
                        for neighbor in World::vec3_neighbors(current.1).into_iter() {
                            queue.push_back((current.0 + 1, *neighbor));
                        }
                    }
                }
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

            for r in redstone.into_iter() {
                seen.insert(r as *const Redstone);
            }

            cgs.push(ConstraintGraph::collect(redstone));
        }

        cgs
    }

    fn is_linkable_from_torch(&self, torch: (Vec3, &TorchVoxel), other: (Vec3, &Voxel)) -> bool {
        let placed_on_vec3 = match &torch.1.facing {
            Some(Facing::North) => torch.0.north(),
            Some(Facing::East) => torch.0.east(),
            Some(Facing::West) => torch.0.west(),
            Some(Facing::South) => torch.0.south(),
            None => torch.0.down(),
        };

        // The torch is never linked to the voxel for which it is placed upon.
        if other.0 == placed_on_vec3 {
            return false;
        }

        match (&other.1, (torch.0 - other.0)) {
            (Voxel::Air(_), _) => false,
            (Voxel::Stone(_), Vec3(_, 1, _)) => true,
            (Voxel::Stone(_), Vec3(_, _, _)) => false,
            (Voxel::Torch(_), Vec3(_, 0, _)) => false,
            (Voxel::Dust(_), _) => true,
            (_, _) => false,
        }
    }

    fn visit_torch_voxel(&'r self, vec3: Vec3, torch: &TorchVoxel) {
        let redstone = self.get(vec3).unwrap();

        for neighbor in self
            .neighbors(vec3)
            .into_iter()
            .filter(|n| self.is_linkable_from_torch((vec3, torch), (n.vec3, n.voxel)))
        {
            if let Some(target) = neighbor.redstone {
                redstone.link(target);
            }
        }
    }

    fn is_linkable_from_dust(&self, dust: (Vec3, &DustVoxel), other: (Vec3, &Voxel)) -> bool {
        match (dust.0 - other.0).abs() {
            Vec3(0, 0, 0) => panic!("Same voxel?"),
            Vec3(0, 0, 1) if other.1.is_dust() => true,
            Vec3(0, 1, 0) if other.1.is_dust() => true,
            Vec3(1, 0, 0) if other.1.is_dust() => true,
            _ => false,
        }
    }

    fn visit_dust_voxel(&'r self, vec3: Vec3, dust: &DustVoxel) {
        let redstone = self.get(vec3).unwrap();
        let neighbors = self.neighbors(vec3);

        // A dust must be placed on a stone at all times.
        assert!(neighbors.down().voxel.is_stone());
        redstone.link(neighbors.down().redstone.unwrap());

        let (viable, nonviable): (Vec<&VoxelCtxt>, Vec<&VoxelCtxt>) = neighbors
            .into_iter()
            .partition(|n| self.is_linkable_from_dust((vec3, dust), (n.vec3, n.voxel)));

        for ctxt in viable {
            redstone.link(ctxt.redstone.unwrap());
        }

        if neighbors.up().voxel.is_air() {
            for ctxt in nonviable.iter().filter(|ctx| ctx.voxel.is_dust()) {
                redstone.link(ctxt.redstone.unwrap());
            }
        }
    }
}
