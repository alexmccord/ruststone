use ruststone::{vec3::Vec3, voxels::Voxel, world::World};

#[test]
fn subscripting() {
    let world = World::new();

    assert!(world[Vec3(0, 0, 0)].is_air());
}

#[test]
fn subscripting_mut() {
    let mut world = World::new();
    world[Vec3(0, 0, 0)] = Voxel::stone().voxel();

    assert!(world[Vec3(0, 0, 0)].is_stone());
}

#[test]
fn put_a_torch_on() {
    let mut world = World::new();

    world[Vec3(0, 1, 0)] = Voxel::torch().voxel();
    world[Vec3(0, 0, 0)] = Voxel::stone().voxel();

    assert!(world[Vec3(0, 1, 0)].is_torch());
    assert!(world[Vec3(0, 0, 0)].is_stone());
}

#[test]
fn identical_vec3s() {
    let mut world = World::new();

    world[Vec3(0, 1, 2)] = Voxel::torch().voxel();
    world[Vec3(2, 1, 0)] = Voxel::stone().voxel();

    assert!(world[Vec3(0, 1, 2)].is_torch());
    assert!(world[Vec3(2, 1, 0)].is_stone());
}
