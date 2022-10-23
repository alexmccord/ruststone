use ruststone::redstone::{RedstoneDust, RedstoneLogic, RedstoneTorch, Redpower};

#[test]
fn torch_and_dust() {
    let mut torch = RedstoneTorch::new();
    let dust = RedstoneDust::new();
    torch.connect(dust);
    torch.apply();

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust.redpower(), Redpower::new(15));
}
