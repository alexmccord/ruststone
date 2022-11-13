use ruststone::{vec3::Vec3, voxels::Voxel, world::World};

#[test]
fn subscripting() {
    let world = World::new();

    assert!(matches!(world[Vec3(0, 0, 0)], Voxel::Air));
}

#[test]
fn subscripting_mut() {
    let mut world = World::new();
    world[Vec3(0, 0, 0)] = Voxel::Stone;

    assert!(matches!(world[Vec3(0, 0, 0)], Voxel::Stone));
}

#[test]
fn put_a_torch_on() {
    let mut world = World::new();

    world[Vec3(0, 1, 0)] = Voxel::torch(None);
    world[Vec3(0, 0, 0)] = Voxel::stone();

    assert!(matches!(world[Vec3(0, 1, 0)], Voxel::Torch(..)));
    assert!(matches!(world[Vec3(0, 0, 0)], Voxel::Stone));
}
