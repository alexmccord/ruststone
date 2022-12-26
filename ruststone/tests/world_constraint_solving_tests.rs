use ruststone::{vec3::Vec3, voxels::Voxel, world::World};

#[test]
fn cyclic_dusts() {
    let mut world = World::new();

    for x in 0..2 {
        for z in 0..2 {
            world[Vec3(x, 0, z)] = Voxel::stone().voxel();
            world[Vec3(x, 1, z)] = Voxel::dust().voxel();
        }
    }

    world.run();
}

#[test]
fn torch_powers_up_the_dust() {
    let mut world = World::new();

    world[Vec3(0, 0, 0)] = Voxel::stone().voxel();
    world[Vec3(0, 1, 0)] = Voxel::torch().voxel();
    world[Vec3(0, 0, 1)] = Voxel::stone().voxel();
    world[Vec3(0, 1, 1)] = Voxel::dust().voxel();

    world.run();

    assert!(world.get(Vec3(0, 0, 0)).unwrap().redstate().is_off());
    assert!(world.get(Vec3(0, 1, 0)).unwrap().redstate().is_on());
    assert!(world.get(Vec3(0, 0, 1)).unwrap().redstate().is_on());
    assert!(world.get(Vec3(0, 1, 1)).unwrap().redstate().is_on());
}

#[test]
fn torch_facing_west_powers_up_the_dust_below() {
    let mut world = World::new();

    world[Vec3(1, 2, 0)] = Voxel::stone().voxel();
    world[Vec3(0, 2, 0)] = Voxel::torch().facing_west().voxel();
    world[Vec3(0, 0, 0)] = Voxel::stone().voxel();
    world[Vec3(0, 1, 0)] = Voxel::dust().voxel();

    world.run();

    assert!(world.get(Vec3(1, 2, 0)).unwrap().redstate().is_off());
    assert!(world.get(Vec3(0, 2, 0)).unwrap().redstate().is_on());
    assert!(world.get(Vec3(0, 0, 0)).unwrap().redstate().is_on());
    assert!(world.get(Vec3(0, 1, 0)).unwrap().redstate().is_on());
}

#[test]
fn and_gate() {
    let mut world = World::new();

    world[Vec3(0, 0, 0)] = Voxel::stone().voxel();
    world[Vec3(0, 1, 0)] = Voxel::torch().voxel();
    world[Vec3(1, 0, 0)] = Voxel::stone().voxel();
    world[Vec3(1, 1, 0)] = Voxel::dust().voxel();

    world[Vec3(0, 0, 2)] = Voxel::stone().voxel();
    world[Vec3(0, 1, 2)] = Voxel::torch().voxel();
    world[Vec3(1, 0, 2)] = Voxel::stone().voxel();
    world[Vec3(1, 1, 2)] = Voxel::dust().voxel();

    world[Vec3(2, 1, 0)] = Voxel::stone().voxel();
    world[Vec3(2, 2, 0)] = Voxel::torch().voxel();
    world[Vec3(2, 1, 1)] = Voxel::stone().voxel();
    world[Vec3(2, 2, 1)] = Voxel::dust().voxel();
    world[Vec3(2, 1, 2)] = Voxel::stone().voxel();
    world[Vec3(2, 2, 2)] = Voxel::torch().voxel();

    world[Vec3(3, 1, 1)] = Voxel::torch().facing_east().voxel();
    world[Vec3(4, 0, 1)] = Voxel::stone().voxel();
    world[Vec3(4, 1, 1)] = Voxel::dust().voxel();

    world.run();

    assert!(world.get(Vec3(0, 1, 0)).unwrap().redstate().is_on());
    assert!(world.get(Vec3(0, 1, 2)).unwrap().redstate().is_on());
}

#[test]
fn double_torches() {
    let mut world = World::new();

    world[Vec3(0, 0, 0)] = Voxel::stone().voxel();
    world[Vec3(0, 1, 0)] = Voxel::torch().voxel();
    world[Vec3(1, 0, 0)] = Voxel::stone().voxel();
    world[Vec3(1, 1, 0)] = Voxel::torch().voxel();

    world.run();

    assert!(world.get(Vec3(0, 1, 0)).unwrap().redstate().is_on());
    assert!(world.get(Vec3(1, 1, 0)).unwrap().redstate().is_on());
}
