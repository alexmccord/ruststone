use ruststone::{ConstraintGraph, Redstone};

#[test]
fn torch_and_dust() {
    let torch = Redstone::torch();
    let dust = Redstone::dust();

    ruststone::link(&torch, &dust);

    let cg = ConstraintGraph::collect(torch.clone());
    assert_eq!(cg.len(), 1);
    cg.solve_constraints();

    assert_eq!(torch.borrow().redstate().get_power(), 16);
    assert_eq!(dust.borrow().redstate().get_power(), 15);
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

    let cg = ConstraintGraph::collect(torch.clone());
    assert_eq!(cg.len(), 1);
    cg.solve_constraints();

    assert_eq!(torch.borrow().redstate().get_power(), 16);
    assert_eq!(dust1.borrow().redstate().get_power(), 15);
    assert_eq!(dust2.borrow().redstate().get_power(), 14);
    assert_eq!(dust3.borrow().redstate().get_power(), 13);
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

    let cg = ConstraintGraph::collect(torch.clone());
    assert_eq!(cg.len(), 1);
    cg.solve_constraints();

    assert_eq!(torch.borrow().redstate().get_power(), 16);
    assert_eq!(dust15.borrow().redstate().get_power(), 1);
    assert_eq!(dust16.borrow().redstate().get_power(), 0);
    assert_eq!(dust17.borrow().redstate().get_power(), 0);
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

    let cg = ConstraintGraph::collect(torch_l.clone());
    assert_eq!(cg.len(), 2);
    cg.solve_constraints();

    assert_eq!(torch_l.borrow().redstate().get_power(), 16);
    assert_eq!(dust1.borrow().redstate().get_power(), 15);
    assert_eq!(dust2.borrow().redstate().get_power(), 14);
    assert_eq!(dust3.borrow().redstate().get_power(), 13);
    assert_eq!(dust4.borrow().redstate().get_power(), 14);
    assert_eq!(dust5.borrow().redstate().get_power(), 15);
    assert_eq!(torch_r.borrow().redstate().get_power(), 16);
}

#[test]
fn torch_is_off_if_its_incoming_edge_is_on() {
    let torch = Redstone::torch();
    let dust = Redstone::dust();
    let normal_block = Redstone::normal_block();
    let output = Redstone::torch();

    ruststone::link(&torch, &dust);
    ruststone::link(&dust, &normal_block);
    ruststone::link(&normal_block, &output);

    let cg = ConstraintGraph::collect(torch.clone());
    assert_eq!(cg.len(), 2);
    cg.solve_constraints();

    assert_eq!(torch.borrow().redstate().get_power(), 16);
    assert_eq!(dust.borrow().redstate().get_power(), 15);
    assert_eq!(normal_block.borrow().redstate().get_power(), 0);
    assert!(normal_block.borrow().redstate().is_forced());
    assert_eq!(output.borrow().redstate().get_power(), 0);
}

#[test]
fn torch_and_dust_and_block_and_dust() {
    let torch = Redstone::torch();
    let dust1 = Redstone::dust();
    let normal_block = Redstone::normal_block();
    let dust2 = Redstone::dust();

    ruststone::link(&torch, &dust1);
    ruststone::link(&dust1, &normal_block);
    ruststone::link(&normal_block, &dust2);

    let cg = ConstraintGraph::collect(torch.clone());
    assert_eq!(cg.len(), 1);
    cg.solve_constraints();

    assert_eq!(torch.borrow().redstate().get_power(), 16);
    assert_eq!(dust1.borrow().redstate().get_power(), 15);
    assert_eq!(normal_block.borrow().redstate().get_power(), 0);
    assert!(normal_block.borrow().redstate().is_forced());
    assert_eq!(dust2.borrow().redstate().get_power(), 0);
}
