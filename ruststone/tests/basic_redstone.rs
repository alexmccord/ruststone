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


#[test]
fn torch_and_dust_and_dust_and_dust() {
    let torch = RedstoneTorch::new();
    let dust1 = RedstoneDust::new();
    let dust2 = RedstoneDust::new();
    let dust3 = RedstoneDust::new();
    
    torch.link(dust1.clone());
    dust1.link(dust2.clone());
    dust2.link(dust3.clone());

    torch.apply();

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust1.redpower(), Redpower::new(15));
    assert_eq!(dust2.redpower(), Redpower::new(14));
    assert_eq!(dust3.redpower(), Redpower::new(13));
}

#[test]
fn dust_in_the_middle_of_two_torches() {
    let torch_l = RedstoneTorch::new();
    let dust1 = RedstoneDust::new();
    let dust2 = RedstoneDust::new();
    let dust3 = RedstoneDust::new();
    let torch_r = RedstoneTorch::new();

    torch_l.link(dust1.clone());
    dust1.link(dust2.clone());
    dust2.link(dust3.clone());
    torch_r.link(dust3.clone());

    torch_l.apply();

    assert_eq!(torch_l.redpower(), Redpower::new(16));
    assert_eq!(dust1.redpower(), Redpower::new(15));
    assert_eq!(dust2.redpower(), Redpower::new(14));
    assert_eq!(dust3.redpower(), Redpower::new(15));
    assert_eq!(torch_r.redpower(), Redpower::new(16));
}

#[test]
fn torch_is_off_if_its_incoming_edge_is_on() {
    let torch = RedstoneTorch::new();
    let dust = RedstoneDust::new();
    let output = RedstoneTorch::new();

    torch.link(dust.clone());
    dust.link(output.clone());

    torch.apply();

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust.redpower(), Redpower::new(15));
    assert_eq!(output.redpower(), Redpower::new(0));
}
