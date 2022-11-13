use ruststone::vec3::Vec3;

#[test]
fn add() {
    let a = Vec3(5, 5, 5);
    let b = Vec3(1, 2, 3);

    assert_eq!(a + b, Vec3(6, 7, 8));
}

#[test]
fn sub() {
    let a = Vec3(5, 5, 5);
    let b = Vec3(1, 2, 3);

    assert_eq!(a - b, Vec3(4, 3, 2));
}

#[test]
fn into() {
    let a: Vec3 = (0, 0, 0).into();

    assert_eq!(a, Vec3(0, 0, 0));
}

#[test]
fn display() {
    let a = Vec3(5, 7, 1);

    assert_eq!(a.to_string(), "(5, 7, 1)");
}
