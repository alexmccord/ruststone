use ruststone::{ConstraintGraph, Redstone};

#[test]
fn torch_and_dust() {
    let torch = Redstone::torch();
    let dust = Redstone::dust();

    ruststone::link(&torch, &dust);

    let cgb = ConstraintGraph::collect(torch);
    cgb.solve_constraints();

    assert_eq!(cgb.len(), 2);
    // assert_eq!(torch.redpower(), Some(Redpower::new(16)));
    // assert_eq!(dust.redpower(), Some(Redpower::new(15)));
}

#[test]
fn torch_and_dust_and_dust_and_dust() {
    let torch = Redstone::torch();
    let dust1 = Redstone::dust();
    let dust2 = Redstone::dust();
    let dust3 = Redstone::dust();

    ruststone::link(&torch, &dust1);
    ruststone::link(&dust1, &dust2);
    ruststone::link(&dust2, &dust3);

    let cgb = ConstraintGraph::collect(torch);
    cgb.solve_constraints();

    assert_eq!(cgb.len(), 4);
    // assert_eq!(torch.redpower(), Some(Redpower::new(16)));
    // assert_eq!(dust1.redpower(), Some(Redpower::new(15)));
    // assert_eq!(dust2.redpower(), Some(Redpower::new(14)));
    // assert_eq!(dust3.redpower(), Some(Redpower::new(13)));
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

    ruststone::link(&torch, &dust1);
    ruststone::link(&dust1, &dust2);
    ruststone::link(&dust2, &dust3);
    ruststone::link(&dust3, &dust4);
    ruststone::link(&dust4, &dust5);
    ruststone::link(&dust5, &dust6);
    ruststone::link(&dust6, &dust7);
    ruststone::link(&dust7, &dust8);
    ruststone::link(&dust8, &dust9);
    ruststone::link(&dust9, &dust10);
    ruststone::link(&dust10, &dust11);
    ruststone::link(&dust11, &dust12);
    ruststone::link(&dust12, &dust13);
    ruststone::link(&dust13, &dust14);
    ruststone::link(&dust14, &dust15);
    ruststone::link(&dust15, &dust16);
    ruststone::link(&dust16, &dust17);

    let cgb = ConstraintGraph::collect(torch);
    cgb.solve_constraints();

    assert_eq!(cgb.len(), 18);
    // assert_eq!(torch.redpower(), Some(Redpower::new(16)));
    // assert_eq!(dust15.redpower(), Some(Redpower::new(1)));
    // assert_eq!(dust16.redpower(), Some(Redpower::new(0)));
    // assert_eq!(dust17.redpower(), Some(Redpower::new(0)));
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

    ruststone::link(&torch_l, &dust1);
    ruststone::link(&dust1, &dust2);
    ruststone::link(&dust2, &dust3);
    ruststone::link(&dust3, &dust4);
    ruststone::link(&dust4, &dust5);
    ruststone::link(&torch_r, &dust5);

    let cgb = ConstraintGraph::collect(torch_l);
    cgb.solve_constraints();

    assert_eq!(cgb.len(), 7);
    // assert_eq!(torch_l.redpower(), Some(Redpower::new(16)));
    // assert_eq!(dust1.redpower(), Some(Redpower::new(15)));
    // assert_eq!(dust2.redpower(), Some(Redpower::new(14)));
    // assert_eq!(dust3.redpower(), Some(Redpower::new(13)));
    // assert_eq!(dust4.redpower(), Some(Redpower::new(14)));
    // assert_eq!(dust5.redpower(), Some(Redpower::new(15)));
    // assert_eq!(torch_r.redpower(), Some(Redpower::new(16)));
}

#[test]
fn torch_is_off_if_its_incoming_edge_is_on() {
    let torch = Redstone::torch();
    let dust = Redstone::dust();
    let output = Redstone::torch();

    ruststone::link(&torch, &dust);
    ruststone::link(&dust, &output);

    let cgb = ConstraintGraph::collect(torch);
    cgb.solve_constraints();

    assert_eq!(cgb.len(), 3);
    // assert_eq!(torch.redpower(), Some(Redpower::new(16)));
    // assert_eq!(dust.redpower(), Some(Redpower::new(15)));
    // assert_eq!(output.redpower(), Some(Redpower::new(0)));
}
