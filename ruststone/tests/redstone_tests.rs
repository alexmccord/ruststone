use ruststone::{Redpower, Redstone, RedstoneLinking, RedstoneLogic};

#[test]
fn torch_and_dust() {
    let torch = Redstone::torch();
    let dust = Redstone::dust();

    torch.link(&dust);

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust.redpower(), Redpower::new(15));
}

#[test]
fn torch_and_dust_and_dust_and_dust() {
    let torch = Redstone::torch();
    let dust1 = Redstone::dust();
    let dust2 = Redstone::dust();
    let dust3 = Redstone::dust();

    torch.link(&dust1);
    dust1.link(&dust2);
    dust2.link(&dust3);

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust1.redpower(), Redpower::new(15));
    assert_eq!(dust2.redpower(), Redpower::new(14));
    assert_eq!(dust3.redpower(), Redpower::new(13));
}

#[test]
fn torch_and_dust_until_it_runs_out_of_redpower() {
    let torch = Redstone::torch();
    let dust1 = Redstone::dust();
    let dust2 = Redstone::dust();
    let dust3 = Redstone::dust();
    let dust4 = Redstone::dust();
    let dust5 = Redstone::dust();
    let dust6 = Redstone::dust();
    let dust7 = Redstone::dust();
    let dust8 = Redstone::dust();
    let dust9 = Redstone::dust();
    let dust10 = Redstone::dust();
    let dust11 = Redstone::dust();
    let dust12 = Redstone::dust();
    let dust13 = Redstone::dust();
    let dust14 = Redstone::dust();
    let dust15 = Redstone::dust();
    let dust16 = Redstone::dust();
    let dust17 = Redstone::dust();

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

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust15.redpower(), Redpower::new(1));
    assert_eq!(dust16.redpower(), Redpower::new(0));
    assert_eq!(dust17.redpower(), Redpower::new(0));
}

#[test]
fn dust_in_the_middle_of_two_torches() {
    let torch_l = Redstone::torch();
    let dust1 = Redstone::dust();
    let dust2 = Redstone::dust();
    let dust3 = Redstone::dust();
    let dust4 = Redstone::dust();
    let dust5 = Redstone::dust();
    let torch_r = Redstone::torch();

    torch_l.link(&dust1);
    dust1.link(&dust2);
    dust2.link(&dust3);
    dust3.link(&dust4);
    dust4.link(&dust5);
    torch_r.link(&dust5);

    assert_eq!(torch_l.redpower(), Redpower::new(16));
    assert_eq!(dust1.redpower(), Redpower::new(15));
    assert_eq!(dust2.redpower(), Redpower::new(14));
    assert_eq!(dust3.redpower(), Redpower::new(13));
    assert_eq!(dust4.redpower(), Redpower::new(14));
    assert_eq!(dust5.redpower(), Redpower::new(15));
    assert_eq!(torch_r.redpower(), Redpower::new(16));
}

#[test]
fn torch_is_off_if_its_incoming_edge_is_on() {
    let torch = Redstone::torch();
    let dust = Redstone::dust();
    let output = Redstone::torch();

    torch.link(&dust);
    dust.link(&output);

    assert_eq!(torch.redpower(), Redpower::new(16));
    assert_eq!(dust.redpower(), Redpower::new(15));
    assert_eq!(output.redpower(), Redpower::new(0));
}
