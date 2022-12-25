use ruststone::{world::World, voxels::{Voxel, Facing}, vec3::Vec3};

#[test]
fn torch_powers_up_the_dust() {
    let mut world = World::new();

    world[Vec3(0, 0, 0)] = Voxel::stone();
    world[Vec3(0, 1, 0)] = Voxel::torch(None);
    world[Vec3(0, 0, 1)] = Voxel::stone();
    world[Vec3(0, 1, 1)] = Voxel::dust();

    world.run();

    assert!(world.get_redstone_at(Vec3(0, 1, 0)).unwrap().redstate().is_on());
    assert!(world.get_redstone_at(Vec3(0, 1, 1)).unwrap().redstate().is_on());
}

#[test]
fn and_gate() {
    let mut world = World::new();

    world[Vec3(0, 0, 0)] = Voxel::stone();
    world[Vec3(0, 1, 0)] = Voxel::torch(None);
    world[Vec3(1, 0, 0)] = Voxel::stone();
    world[Vec3(1, 1, 0)] = Voxel::dust();

    world[Vec3(0, 0, 2)] = Voxel::stone();
    world[Vec3(0, 1, 2)] = Voxel::torch(None);
    world[Vec3(1, 0, 2)] = Voxel::stone();
    world[Vec3(1, 1, 2)] = Voxel::dust();

    world[Vec3(0, 1, 0)] = Voxel::stone();
    world[Vec3(0, 1, 1)] = Voxel::stone();
    world[Vec3(0, 1, 2)] = Voxel::stone();
    world[Vec3(0, 2, 0)] = Voxel::torch(None);
    world[Vec3(0, 2, 1)] = Voxel::dust();
    world[Vec3(0, 2, 2)] = Voxel::torch(None);

    world[Vec3(0, 1, 3)] = Voxel::torch(Some(Facing::East));
    world[Vec3(0, 0, 4)] = Voxel::stone();
    world[Vec3(0, 1, 4)] = Voxel::dust();

    world.run();

    assert!(world
        .get_redstone_at(Vec3(0, 1, 0))
        .unwrap()
        .redstate()
        .is_on());

    assert!(world
        .get_redstone_at(Vec3(0, 1, 2))
        .unwrap()
        .redstate()
        .is_on());
}
