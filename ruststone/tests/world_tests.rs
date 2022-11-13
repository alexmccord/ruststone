use ruststone::{blocks::Block, vec3::Vec3, world::World};

#[test]
fn subscripting() {
    let world = World::new();

    assert!(matches!(world[Vec3(0, 0, 0)], Block::Air));
}

#[test]
fn subscripting_mut() {
    let mut world = World::new();
    world[Vec3(0, 0, 0)] = Block::Stone;

    assert!(matches!(world[Vec3(0, 0, 0)], Block::Stone));
}
