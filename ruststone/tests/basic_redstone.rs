use ruststone::redstone::{Redpower, RedstoneDust, RedstoneLinking, RedstoneLogic, RedstoneTorch};

#[test]
fn torch_and_dust() {
    let torch = RedstoneTorch::new();
    let dust = RedstoneDust::new();

    torch.link(&dust);

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

    torch.link(&dust1);
    dust1.link(&dust2);
    dust2.link(&dust3);

    torch.apply();

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust1.redpower(), Redpower::new(15));
    assert_eq!(dust2.redpower(), Redpower::new(14));
    assert_eq!(dust3.redpower(), Redpower::new(13));
}

#[test]
fn torch_and_dust_until_it_runs_out_of_redpower() {
    let torch = RedstoneTorch::new();
    let dust1 = RedstoneDust::new();
    let dust2 = RedstoneDust::new();
    let dust3 = RedstoneDust::new();
    let dust4 = RedstoneDust::new();
    let dust5 = RedstoneDust::new();
    let dust6 = RedstoneDust::new();
    let dust7 = RedstoneDust::new();
    let dust8 = RedstoneDust::new();
    let dust9 = RedstoneDust::new();
    let dust10 = RedstoneDust::new();
    let dust11 = RedstoneDust::new();
    let dust12 = RedstoneDust::new();
    let dust13 = RedstoneDust::new();
    let dust14 = RedstoneDust::new();
    let dust15 = RedstoneDust::new();
    let dust16 = RedstoneDust::new();
    let dust17 = RedstoneDust::new();

    torch.link(&dust1);
    dust1.link(&dust2);
    dust2.link(&dust3);
    dust3.link(&dust4);
    dust4.link(&dust5);
    dust5.link(&dust6);
    dust6.link(&dust7);
    dust7.link(&dust8);
    dust8.link(&dust9);
    dust9.link(&dust10);
    dust10.link(&dust11);
    dust11.link(&dust12);
    dust12.link(&dust13);
    dust13.link(&dust14);
    dust14.link(&dust15);
    dust15.link(&dust16);
    dust16.link(&dust17);

    torch.apply();

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust15.redpower(), Redpower::new(1));
    assert_eq!(dust16.redpower(), Redpower::new(0));
    assert_eq!(dust17.redpower(), Redpower::new(0));
}

#[test]
fn dust_in_the_middle_of_two_torches() {
    let torch_l = RedstoneTorch::new();
    let dust1 = RedstoneDust::new();
    let dust2 = RedstoneDust::new();
    let dust3 = RedstoneDust::new();
    let torch_r = RedstoneTorch::new();

    torch_l.link(&dust1);
    dust1.link(&dust2);
    dust2.link(&dust3);
    torch_r.link(&dust3);

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

    torch.link(&dust);
    dust.link(&output);

    torch.apply();

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust.redpower(), Redpower::new(15));
    assert_eq!(output.redpower(), Redpower::new(0));
}
