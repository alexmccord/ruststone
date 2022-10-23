use ruststone::redstone::{Redpower, RedstoneDust, RedstoneLinking, RedstoneLogic, RedstoneTorch};

#[test]
fn torch_and_dust() {
    let torch = RedstoneTorch::new();
    let dust = RedstoneDust::new();
    torch.link(dust.clone());
    torch.apply();

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust.redpower(), Redpower::new(15));
}
