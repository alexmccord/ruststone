use ruststone::{ConstraintGraph, RedstoneArena};

#[test]
fn torch_and_dust() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust = arena.dust("dust");

    ruststone::link(torch, dust);

    ruststone::add_weighted_edge(dust, torch, 1);

    let cg = ConstraintGraph::collect(torch);
    cg.solve_constraints();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust.redstate().get_power(), 15);
}

#[test]
fn torch_and_dust_and_dust_and_dust() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust1 = arena.dust("dust1");
    let dust2 = arena.dust("dust2");
    let dust3 = arena.dust("dust3");

    ruststone::link(torch, dust1);
    ruststone::link(dust1, dust2);
    ruststone::link(dust2, dust3);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, torch, 2);
    ruststone::add_weighted_edge(dust3, torch, 3);

    let cg = ConstraintGraph::collect(torch);
    cg.solve_constraints();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(dust2.redstate().get_power(), 14);
    assert_eq!(dust3.redstate().get_power(), 13);
}

#[test]
fn torch_and_dust_until_it_runs_out_of_redpower() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust1 = arena.dust("dust1");
    let dust2 = arena.dust("dust2");
    let dust3 = arena.dust("dust3");
    let dust4 = arena.dust("dust4");
    let dust5 = arena.dust("dust5");
    let dust6 = arena.dust("dust6");
    let dust7 = arena.dust("dust7");
    let dust8 = arena.dust("dust8");
    let dust9 = arena.dust("dust9");
    let dust10 = arena.dust("dust10");
    let dust11 = arena.dust("dust11");
    let dust12 = arena.dust("dust12");
    let dust13 = arena.dust("dust13");
    let dust14 = arena.dust("dust14");
    let dust15 = arena.dust("dust15");
    let dust16 = arena.dust("dust16");
    let dust17 = arena.dust("dust17");

    ruststone::link(torch, dust1);
    ruststone::link(dust1, dust2);
    ruststone::link(dust2, dust3);
    ruststone::link(dust3, dust4);
    ruststone::link(dust4, dust5);
    ruststone::link(dust5, dust6);
    ruststone::link(dust6, dust7);
    ruststone::link(dust7, dust8);
    ruststone::link(dust8, dust9);
    ruststone::link(dust9, dust10);
    ruststone::link(dust10, dust11);
    ruststone::link(dust11, dust12);
    ruststone::link(dust12, dust13);
    ruststone::link(dust13, dust14);
    ruststone::link(dust14, dust15);
    ruststone::link(dust15, dust16);
    ruststone::link(dust16, dust17);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, torch, 2);
    ruststone::add_weighted_edge(dust3, torch, 3);
    ruststone::add_weighted_edge(dust4, torch, 4);
    ruststone::add_weighted_edge(dust5, torch, 5);
    ruststone::add_weighted_edge(dust6, torch, 6);
    ruststone::add_weighted_edge(dust7, torch, 7);
    ruststone::add_weighted_edge(dust8, torch, 8);
    ruststone::add_weighted_edge(dust9, torch, 9);
    ruststone::add_weighted_edge(dust10, torch, 10);
    ruststone::add_weighted_edge(dust11, torch, 11);
    ruststone::add_weighted_edge(dust12, torch, 12);
    ruststone::add_weighted_edge(dust13, torch, 13);
    ruststone::add_weighted_edge(dust14, torch, 14);
    ruststone::add_weighted_edge(dust15, torch, 15);
    ruststone::add_weighted_edge(dust16, torch, 16);
    ruststone::add_weighted_edge(dust17, torch, 17);

    let cg = ConstraintGraph::collect(torch);
    cg.solve_constraints();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust15.redstate().get_power(), 1);
    assert_eq!(dust16.redstate().get_power(), 0);
    assert_eq!(dust17.redstate().get_power(), 0);
}

#[test]
fn dust_in_the_middle_of_two_torches() {
    let arena = RedstoneArena::new();

    let torch_l = arena.torch("torch_l");
    let dust1 = arena.dust("dust1");
    let dust2 = arena.dust("dust2");
    let dust3 = arena.dust("dust3");
    let dust4 = arena.dust("dust4");
    let dust5 = arena.dust("dust5");
    let torch_r = arena.torch("torch_r");

    ruststone::link(torch_l, dust1);
    ruststone::link(dust1, dust2);
    ruststone::link(dust2, dust3);
    ruststone::link(dust3, dust4);
    ruststone::link(dust4, dust5);
    ruststone::link(torch_r, dust5);

    ruststone::add_weighted_edge(dust1, torch_l, 1);
    ruststone::add_weighted_edge(dust2, torch_l, 2);
    ruststone::add_weighted_edge(dust3, torch_l, 3);
    ruststone::add_weighted_edge(dust4, torch_l, 4);
    ruststone::add_weighted_edge(dust5, torch_l, 5);

    ruststone::add_weighted_edge(dust5, torch_r, 1);
    ruststone::add_weighted_edge(dust4, torch_r, 2);
    ruststone::add_weighted_edge(dust3, torch_r, 3);
    ruststone::add_weighted_edge(dust2, torch_r, 4);
    ruststone::add_weighted_edge(dust1, torch_r, 5);

    let cg = ConstraintGraph::collect(torch_l);
    cg.solve_constraints();

    assert_eq!(torch_l.redstate().get_power(), 16);
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(dust2.redstate().get_power(), 14);
    assert_eq!(dust3.redstate().get_power(), 13);
    assert_eq!(dust4.redstate().get_power(), 14);
    assert_eq!(dust5.redstate().get_power(), 15);
    assert_eq!(torch_r.redstate().get_power(), 16);
}

#[test]
fn torch_is_off_if_its_incoming_edge_is_on() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust = arena.dust("dust");
    let normal_block = arena.block("normal_block");
    let output = arena.torch("output");

    ruststone::link(torch, dust);
    ruststone::link(dust, normal_block);
    ruststone::link(normal_block, output);

    ruststone::add_weighted_edge(dust, torch, 1);

    let cg = ConstraintGraph::collect(torch);
    cg.solve_constraints();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust.redstate().get_power(), 15);
    assert_eq!(normal_block.redstate().get_power(), 0);
    assert!(normal_block.redstate().is_forced());
    assert_eq!(output.redstate().get_power(), 0);
}

#[test]
fn torch_and_dust_and_block_and_dust() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust1 = arena.dust("dust1");
    let normal_block = arena.block("normal_block");
    let dust2 = arena.dust("dust2");

    ruststone::link(torch, dust1);
    ruststone::link(dust1, normal_block);
    ruststone::link(normal_block, dust2);

    ruststone::add_weighted_edge(dust1, torch, 1);

    let cg = ConstraintGraph::collect(torch);
    cg.solve_constraints();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(normal_block.redstate().get_power(), 0);
    assert!(normal_block.redstate().is_forced());
    assert_eq!(dust2.redstate().get_power(), 0);
}

#[test]
fn and_gate() {
    let arena = RedstoneArena::new();

    let input_l = arena.torch("input_l");
    let input_r = arena.torch("input_r");
    let and_l = arena.torch("and_l");
    let and_r = arena.torch("and_r");
    let output = arena.torch("output");

    let dust_l = arena.dust("dust_l");
    let dust_m = arena.dust("dust_m");
    let dust_r = arena.dust("dust_r");

    let block_l = arena.block("block_l");
    let block_m = arena.block("block_m");
    let block_r = arena.block("block_r");

    ruststone::link(input_l, dust_l);
    ruststone::link(dust_l, block_l);
    ruststone::link(block_l, and_l);

    ruststone::link(input_r, dust_r);
    ruststone::link(dust_r, block_r);
    ruststone::link(block_r, and_r);

    ruststone::link(and_l, dust_m);
    ruststone::link(and_r, dust_m);
    ruststone::link(dust_m, block_m);

    ruststone::link(block_m, output);

    ruststone::add_weighted_edge(dust_l, input_l, 1);
    ruststone::add_weighted_edge(dust_r, input_r, 1);
    ruststone::add_weighted_edge(dust_m, and_l, 1);
    ruststone::add_weighted_edge(dust_m, and_r, 1);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(input_l.redstate().is_on());
    assert!(input_r.redstate().is_on());
    assert!(and_l.redstate().is_off());
    assert!(and_r.redstate().is_off());
    assert!(output.redstate().is_on());
}

#[test]
fn and_gate_with_one_arm_off() {
    let arena = RedstoneArena::new();

    let input_r = arena.torch("input_r");
    let and_l = arena.torch("and_l");
    let and_r = arena.torch("and_r");
    let output = arena.torch("output");

    let dust_l = arena.dust("dust_l");
    let dust_m = arena.dust("dust_m");
    let dust_r = arena.dust("dust_r");

    let block_l = arena.block("block_l");
    let block_m = arena.block("block_m");
    let block_r = arena.block("block_r");

    ruststone::link(dust_l, block_l);
    ruststone::link(block_l, and_l);

    ruststone::link(input_r, dust_r);
    ruststone::link(dust_r, block_r);
    ruststone::link(block_r, and_r);

    ruststone::link(and_l, dust_m);
    ruststone::link(and_r, dust_m);
    ruststone::link(dust_m, block_m);

    ruststone::link(block_m, output);

    ruststone::add_weighted_edge(dust_r, input_r, 1);
    ruststone::add_weighted_edge(dust_m, and_l, 1);
    ruststone::add_weighted_edge(dust_m, and_r, 1);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(input_r.redstate().is_on());
    assert!(and_l.redstate().is_on());
    assert!(and_r.redstate().is_off());
    assert!(output.redstate().is_off());
}

#[test]
fn and_gate_with_both_arms_off() {
    let arena = RedstoneArena::new();

    let and_l = arena.torch("and_l");
    let and_r = arena.torch("and_r");
    let output = arena.torch("output");

    let dust_l = arena.dust("dust_l");
    let dust_m = arena.dust("dust_m");
    let dust_r = arena.dust("dust_r");

    let block_l = arena.block("block_l");
    let block_m = arena.block("block_m");
    let block_r = arena.block("block_r");

    ruststone::link(dust_l, block_l);
    ruststone::link(block_l, and_l);

    ruststone::link(dust_r, block_r);
    ruststone::link(block_r, and_r);

    ruststone::link(and_l, dust_m);
    ruststone::link(and_r, dust_m);
    ruststone::link(dust_m, block_m);

    ruststone::link(block_m, output);

    ruststone::add_weighted_edge(dust_m, and_l, 1);
    ruststone::add_weighted_edge(dust_m, and_r, 1);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(and_l.redstate().is_on());
    assert!(and_r.redstate().is_on());
    assert!(output.redstate().is_off());
}

#[test]
fn xor_gate() {
    let arena = RedstoneArena::new();

    let input_l = arena.torch("input_l");
    let input_dust_l = arena.dust("input_dust_l");
    let dust_block_l = arena.block("dust_block_l");
    let torch_on_top_block_l = arena.torch("torch_on_top_block_l");
    let torch_in_front_block_l = arena.torch("torch_in_front_block_l");
    let dust_after_inversion_l = arena.dust("dust_after_inversion_l");
    let dust_after_inversion_l2 = arena.dust("dust_after_inversion_l2");
    let block_after_inversion_l = arena.block("block_after_inversion_l");
    let torch_after_dust_inversion_l = arena.torch("torch_after_dust_inversion_l");

    ruststone::link(input_l, input_dust_l);
    ruststone::link(input_dust_l, dust_block_l);
    ruststone::link(dust_block_l, torch_on_top_block_l);
    ruststone::link(dust_block_l, torch_in_front_block_l);
    ruststone::link(torch_in_front_block_l, dust_after_inversion_l);
    ruststone::link(dust_after_inversion_l, dust_after_inversion_l2);
    ruststone::link(dust_after_inversion_l2, block_after_inversion_l);
    ruststone::link(block_after_inversion_l, torch_after_dust_inversion_l);

    ruststone::add_weighted_edge(input_dust_l, input_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, torch_in_front_block_l, 2);

    let input_r = arena.torch("input_r");
    let input_dust_r = arena.dust("input_dust_r");
    let dust_block_r = arena.block("dust_block_r");
    let torch_on_top_block_r = arena.torch("torch_on_top_block_r");
    let torch_in_front_block_r = arena.torch("torch_in_front_block_r");
    let dust_after_inversion_r = arena.dust("dust_after_inversion_r");
    let dust_after_inversion_r2 = arena.dust("dust_after_inversion_r2");
    let block_after_inversion_r = arena.block("block_after_inversion_r");
    let torch_after_dust_inversion_r = arena.torch("torch_after_dust_inversion_r");

    ruststone::link(input_r, input_dust_r);
    ruststone::link(input_dust_r, dust_block_r);
    ruststone::link(dust_block_r, torch_on_top_block_r);
    ruststone::link(dust_block_r, torch_in_front_block_r);
    ruststone::link(torch_in_front_block_r, dust_after_inversion_r);
    ruststone::link(dust_after_inversion_r, dust_after_inversion_r2);
    ruststone::link(dust_after_inversion_r2, block_after_inversion_r);
    ruststone::link(block_after_inversion_r, torch_after_dust_inversion_r);

    ruststone::add_weighted_edge(input_dust_r, input_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, torch_in_front_block_r, 2);

    let and_dust_1 = arena.dust("and_dust_1");
    let and_dust_2 = arena.dust("and_dust_2");
    let and_block = arena.block("and_block");
    let inversion_of_and = arena.torch("inversion_of_and");

    ruststone::link(torch_on_top_block_l, and_dust_1);
    ruststone::link(torch_on_top_block_r, and_dust_1);
    ruststone::link(and_dust_1, and_dust_2);
    ruststone::link(and_dust_2, and_block);
    ruststone::link(and_block, inversion_of_and);

    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_l, 2);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_r, 2);

    ruststone::link(inversion_of_and, dust_after_inversion_l);
    ruststone::link(inversion_of_and, dust_after_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_l, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, inversion_of_and, 2);
    ruststone::add_weighted_edge(dust_after_inversion_r, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, inversion_of_and, 2);

    let output = arena.dust("output");

    ruststone::link(torch_after_dust_inversion_l, output);
    ruststone::link(torch_after_dust_inversion_r, output);

    ruststone::add_weighted_edge(output, torch_after_dust_inversion_l, 1);
    ruststone::add_weighted_edge(output, torch_after_dust_inversion_r, 1);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(input_l.redstate().is_on());
    assert!(input_r.redstate().is_on());
    assert!(torch_on_top_block_l.redstate().is_off());
    assert!(torch_on_top_block_r.redstate().is_off());
    assert!(torch_in_front_block_l.redstate().is_off());
    assert!(torch_in_front_block_r.redstate().is_off());
    assert!(inversion_of_and.redstate().is_on());
    assert!(torch_after_dust_inversion_l.redstate().is_off());
    assert!(torch_after_dust_inversion_r.redstate().is_off());
    assert!(output.redstate().is_off());
}

#[test]
fn xor_gate_with_left_off() {
    let arena = RedstoneArena::new();

    let input_dust_l = arena.dust("input_dust_l");
    let dust_block_l = arena.block("dust_block_l");
    let torch_on_top_block_l = arena.torch("torch_on_top_block_l");
    let torch_in_front_block_l = arena.torch("torch_in_front_block_l");
    let dust_after_inversion_l = arena.dust("dust_after_inversion_l");
    let dust_after_inversion_l2 = arena.dust("dust_after_inversion_l2");
    let block_after_inversion_l = arena.block("block_after_inversion_l");
    let torch_after_dust_inversion_l = arena.torch("torch_after_dust_inversion_l");

    ruststone::link(input_dust_l, dust_block_l);
    ruststone::link(dust_block_l, torch_on_top_block_l);
    ruststone::link(dust_block_l, torch_in_front_block_l);
    ruststone::link(torch_in_front_block_l, dust_after_inversion_l);
    ruststone::link(dust_after_inversion_l, dust_after_inversion_l2);
    ruststone::link(dust_after_inversion_l2, block_after_inversion_l);
    ruststone::link(block_after_inversion_l, torch_after_dust_inversion_l);

    ruststone::add_weighted_edge(dust_after_inversion_l, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, torch_in_front_block_l, 2);

    let input_r = arena.torch("input_r");
    let input_dust_r = arena.dust("input_dust_r");
    let dust_block_r = arena.block("dust_block_r");
    let torch_on_top_block_r = arena.torch("torch_on_top_block_r");
    let torch_in_front_block_r = arena.torch("torch_in_front_block_r");
    let dust_after_inversion_r = arena.dust("dust_after_inversion_r");
    let dust_after_inversion_r2 = arena.dust("dust_after_inversion_r2");
    let block_after_inversion_r = arena.block("block_after_inversion_r");
    let torch_after_dust_inversion_r = arena.torch("torch_after_dust_inversion_r");

    ruststone::link(input_r, input_dust_r);
    ruststone::link(input_dust_r, dust_block_r);
    ruststone::link(dust_block_r, torch_on_top_block_r);
    ruststone::link(dust_block_r, torch_in_front_block_r);
    ruststone::link(torch_in_front_block_r, dust_after_inversion_r);
    ruststone::link(dust_after_inversion_r, dust_after_inversion_r2);
    ruststone::link(dust_after_inversion_r2, block_after_inversion_r);
    ruststone::link(block_after_inversion_r, torch_after_dust_inversion_r);

    ruststone::add_weighted_edge(input_dust_r, input_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, torch_in_front_block_r, 2);

    let and_dust_1 = arena.dust("and_dust_1");
    let and_dust_2 = arena.dust("and_dust_2");
    let and_block = arena.block("and_block");
    let inversion_of_and = arena.torch("inversion_of_and");

    ruststone::link(torch_on_top_block_l, and_dust_1);
    ruststone::link(torch_on_top_block_r, and_dust_1);
    ruststone::link(and_dust_1, and_dust_2);
    ruststone::link(and_dust_2, and_block);
    ruststone::link(and_block, inversion_of_and);

    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_l, 2);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_r, 2);

    ruststone::link(inversion_of_and, dust_after_inversion_l);
    ruststone::link(inversion_of_and, dust_after_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_l, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, inversion_of_and, 2);
    ruststone::add_weighted_edge(dust_after_inversion_r, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, inversion_of_and, 2);

    let output = arena.dust("output");

    ruststone::link(torch_after_dust_inversion_l, output);
    ruststone::link(torch_after_dust_inversion_r, output);

    ruststone::add_weighted_edge(output, torch_after_dust_inversion_l, 1);
    ruststone::add_weighted_edge(output, torch_after_dust_inversion_r, 1);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(input_r.redstate().is_on());
    assert!(torch_on_top_block_l.redstate().is_on());
    assert!(torch_on_top_block_r.redstate().is_off());
    assert!(torch_in_front_block_l.redstate().is_on());
    assert!(torch_in_front_block_r.redstate().is_off());
    assert!(inversion_of_and.redstate().is_off());
    assert!(torch_after_dust_inversion_l.redstate().is_off());
    assert!(torch_after_dust_inversion_r.redstate().is_on());
    assert!(output.redstate().is_on());
}

#[test]
fn xor_gate_with_right_off() {
    let arena = RedstoneArena::new();

    let input_l = arena.torch("input_l");
    let input_dust_l = arena.dust("input_dust_l");
    let dust_block_l = arena.block("dust_block_l");
    let torch_on_top_block_l = arena.torch("torch_on_top_block_l");
    let torch_in_front_block_l = arena.torch("torch_in_front_block_l");
    let dust_after_inversion_l = arena.dust("dust_after_inversion_l");
    let dust_after_inversion_l2 = arena.dust("dust_after_inversion_l2");
    let block_after_inversion_l = arena.block("block_after_inversion_l");
    let torch_after_dust_inversion_l = arena.torch("torch_after_dust_inversion_l");

    ruststone::link(input_l, input_dust_l);
    ruststone::link(input_dust_l, dust_block_l);
    ruststone::link(dust_block_l, torch_on_top_block_l);
    ruststone::link(dust_block_l, torch_in_front_block_l);
    ruststone::link(torch_in_front_block_l, dust_after_inversion_l);
    ruststone::link(dust_after_inversion_l, dust_after_inversion_l2);
    ruststone::link(dust_after_inversion_l2, block_after_inversion_l);
    ruststone::link(block_after_inversion_l, torch_after_dust_inversion_l);

    ruststone::add_weighted_edge(input_dust_l, input_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, torch_in_front_block_l, 2);

    let input_dust_r = arena.dust("input_dust_r");
    let dust_block_r = arena.block("dust_block_r");
    let torch_on_top_block_r = arena.torch("torch_on_top_block_r");
    let torch_in_front_block_r = arena.torch("torch_in_front_block_r");
    let dust_after_inversion_r = arena.dust("dust_after_inversion_r");
    let dust_after_inversion_r2 = arena.dust("dust_after_inversion_r2");
    let block_after_inversion_r = arena.block("block_after_inversion_r");
    let torch_after_dust_inversion_r = arena.torch("torch_after_dust_inversion_r");

    ruststone::link(input_dust_r, dust_block_r);
    ruststone::link(dust_block_r, torch_on_top_block_r);
    ruststone::link(dust_block_r, torch_in_front_block_r);
    ruststone::link(torch_in_front_block_r, dust_after_inversion_r);
    ruststone::link(dust_after_inversion_r, dust_after_inversion_r2);
    ruststone::link(dust_after_inversion_r2, block_after_inversion_r);
    ruststone::link(block_after_inversion_r, torch_after_dust_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_r, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, torch_in_front_block_r, 2);

    let and_dust_1 = arena.dust("and_dust_1");
    let and_dust_2 = arena.dust("and_dust_2");
    let and_block = arena.block("and_block");
    let inversion_of_and = arena.torch("inversion_of_and");

    ruststone::link(torch_on_top_block_l, and_dust_1);
    ruststone::link(torch_on_top_block_r, and_dust_1);
    ruststone::link(and_dust_1, and_dust_2);
    ruststone::link(and_dust_2, and_block);
    ruststone::link(and_block, inversion_of_and);

    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_l, 2);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_r, 2);

    ruststone::link(inversion_of_and, dust_after_inversion_l);
    ruststone::link(inversion_of_and, dust_after_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_l, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, inversion_of_and, 2);
    ruststone::add_weighted_edge(dust_after_inversion_r, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, inversion_of_and, 2);

    let output = arena.dust("output");

    ruststone::link(torch_after_dust_inversion_l, output);
    ruststone::link(torch_after_dust_inversion_r, output);

    ruststone::add_weighted_edge(output, torch_after_dust_inversion_l, 1);
    ruststone::add_weighted_edge(output, torch_after_dust_inversion_r, 1);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(input_l.redstate().is_on());
    assert!(torch_on_top_block_l.redstate().is_off());
    assert!(torch_on_top_block_r.redstate().is_on());
    assert!(torch_in_front_block_l.redstate().is_off());
    assert!(torch_in_front_block_r.redstate().is_on());
    assert!(inversion_of_and.redstate().is_off());
    assert!(torch_after_dust_inversion_l.redstate().is_on());
    assert!(torch_after_dust_inversion_r.redstate().is_off());
    assert!(output.redstate().is_on());
}

#[test]
fn xor_gate_with_both_off() {
    let arena = RedstoneArena::new();

    let input_dust_l = arena.dust("input_dust_l");
    let dust_block_l = arena.block("dust_block_l");
    let torch_on_top_block_l = arena.torch("torch_on_top_block_l");
    let torch_in_front_block_l = arena.torch("torch_in_front_block_l");
    let dust_after_inversion_l = arena.dust("dust_after_inversion_l");
    let dust_after_inversion_l2 = arena.dust("dust_after_inversion_l2");
    let block_after_inversion_l = arena.block("block_after_inversion_l");
    let torch_after_dust_inversion_l = arena.torch("torch_after_dust_inversion_l");

    ruststone::link(input_dust_l, dust_block_l);
    ruststone::link(dust_block_l, torch_on_top_block_l);
    ruststone::link(dust_block_l, torch_in_front_block_l);
    ruststone::link(torch_in_front_block_l, dust_after_inversion_l);
    ruststone::link(dust_after_inversion_l, dust_after_inversion_l2);
    ruststone::link(dust_after_inversion_l2, block_after_inversion_l);
    ruststone::link(block_after_inversion_l, torch_after_dust_inversion_l);

    ruststone::add_weighted_edge(dust_after_inversion_l, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, torch_in_front_block_l, 2);

    let input_dust_r = arena.dust("input_dust_r");
    let dust_block_r = arena.block("dust_block_r");
    let torch_on_top_block_r = arena.torch("torch_on_top_block_r");
    let torch_in_front_block_r = arena.torch("torch_in_front_block_r");
    let dust_after_inversion_r = arena.dust("dust_after_inversion_r");
    let dust_after_inversion_r2 = arena.dust("dust_after_inversion_r2");
    let block_after_inversion_r = arena.block("block_after_inversion_r");
    let torch_after_dust_inversion_r = arena.torch("torch_after_dust_inversion_r");

    ruststone::link(input_dust_r, dust_block_r);
    ruststone::link(dust_block_r, torch_on_top_block_r);
    ruststone::link(dust_block_r, torch_in_front_block_r);
    ruststone::link(torch_in_front_block_r, dust_after_inversion_r);
    ruststone::link(dust_after_inversion_r, dust_after_inversion_r2);
    ruststone::link(dust_after_inversion_r2, block_after_inversion_r);
    ruststone::link(block_after_inversion_r, torch_after_dust_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_r, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, torch_in_front_block_r, 2);

    let and_dust_1 = arena.dust("and_dust_1");
    let and_dust_2 = arena.dust("and_dust_2");
    let and_block = arena.block("and_block");
    let inversion_of_and = arena.torch("inversion_of_and");

    ruststone::link(torch_on_top_block_l, and_dust_1);
    ruststone::link(torch_on_top_block_r, and_dust_1);
    ruststone::link(and_dust_1, and_dust_2);
    ruststone::link(and_dust_2, and_block);
    ruststone::link(and_block, inversion_of_and);

    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_l, 2);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_r, 2);

    ruststone::link(inversion_of_and, dust_after_inversion_l);
    ruststone::link(inversion_of_and, dust_after_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_l, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, inversion_of_and, 2);
    ruststone::add_weighted_edge(dust_after_inversion_r, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, inversion_of_and, 2);

    let output = arena.dust("output");

    ruststone::link(torch_after_dust_inversion_l, output);
    ruststone::link(torch_after_dust_inversion_r, output);

    ruststone::add_weighted_edge(output, torch_after_dust_inversion_l, 1);
    ruststone::add_weighted_edge(output, torch_after_dust_inversion_r, 1);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(torch_on_top_block_l.redstate().is_on());
    assert!(torch_on_top_block_r.redstate().is_on());
    assert!(torch_in_front_block_l.redstate().is_on());
    assert!(torch_in_front_block_r.redstate().is_on());
    assert!(inversion_of_and.redstate().is_off());
    assert!(torch_after_dust_inversion_l.redstate().is_off());
    assert!(torch_after_dust_inversion_r.redstate().is_off());
    assert!(output.redstate().is_off());
}

#[test]
fn memory_cell() {
    let arena = RedstoneArena::new();

    let block_a = arena.block("block_a");
    let torch_a = arena.torch("torch_a");
    let dust_a1 = arena.dust("dust_a1");
    let dust_a2 = arena.dust("dust_a2");

    let block_b = arena.block("block_b");
    let torch_b = arena.torch("torch_b");
    let dust_b1 = arena.dust("dust_b1");
    let dust_b2 = arena.dust("dust_b2");

    ruststone::link(block_a, torch_a);
    ruststone::link(torch_a, dust_a1);
    ruststone::link(dust_a1, dust_a2);
    ruststone::link(dust_a2, block_b);

    ruststone::add_weighted_edge(dust_a1, torch_a, 1);
    ruststone::add_weighted_edge(dust_a2, torch_a, 2);

    ruststone::link(block_b, torch_b);
    ruststone::link(torch_b, dust_b1);
    ruststone::link(dust_b1, dust_b2);
    ruststone::link(dust_b2, block_a);

    ruststone::add_weighted_edge(dust_b1, torch_b, 1);
    ruststone::add_weighted_edge(dust_b2, torch_b, 2);

    let cg = ConstraintGraph::collect(block_a);
    cg.solve_constraints();

    assert!(torch_a.redstate().is_on());
    assert!(torch_b.redstate().is_off());
}

#[test]
fn memory_cell_alt() {
    let arena = RedstoneArena::new();

    let block_a = arena.block("block_a");
    let torch_a = arena.torch("torch_a");
    let dust_a1 = arena.dust("dust_a1");
    let dust_a2 = arena.dust("dust_a2");

    let block_b = arena.block("block_b");
    let torch_b = arena.torch("torch_b");
    let dust_b1 = arena.dust("dust_b1");
    let dust_b2 = arena.dust("dust_b2");

    ruststone::link(block_a, torch_a);
    ruststone::link(torch_a, dust_a1);
    ruststone::link(dust_a1, dust_a2);
    ruststone::link(dust_a2, block_b);

    ruststone::add_weighted_edge(dust_a1, torch_a, 1);
    ruststone::add_weighted_edge(dust_a2, torch_a, 2);

    ruststone::link(block_b, torch_b);
    ruststone::link(torch_b, dust_b1);
    ruststone::link(dust_b1, dust_b2);
    ruststone::link(dust_b2, block_a);

    ruststone::add_weighted_edge(dust_b1, torch_b, 1);
    ruststone::add_weighted_edge(dust_b2, torch_b, 2);

    let cg = ConstraintGraph::collect(block_b);
    cg.solve_constraints();

    assert!(torch_a.redstate().is_off());
    assert!(torch_b.redstate().is_on());
}

#[test]
fn torch_and_dust_and_block_and_repeater() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust = arena.dust("dust");
    let block = arena.block("block");
    let repeater = arena.repeater("repeater", 1);

    ruststone::link(torch, dust);
    ruststone::link(dust, block);
    ruststone::link(block, repeater);

    ruststone::add_weighted_edge(dust, torch, 1);

    let cg = ConstraintGraph::collect(torch);
    cg.solve_constraints();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust.redstate().get_power(), 15);
    assert_eq!(block.redstate().get_power(), 0);
    assert!(block.redstate().is_forced());
    assert_eq!(repeater.redstate().get_power(), 16);
}

#[test]
fn torch_and_dust_and_block_and_repeater_and_block_and_dust() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust1 = arena.dust("dust1");
    let block1 = arena.block("block1");
    let repeater = arena.repeater("repeater", 1);
    let block2 = arena.block("block2");
    let dust2 = arena.dust("dust2");

    ruststone::link(torch, dust1);
    ruststone::link(dust1, block1);
    ruststone::link(block1, repeater);
    ruststone::link(repeater, block2);
    ruststone::link(block2, dust2);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, block2, 1);

    let cg = ConstraintGraph::collect(torch);
    cg.solve_constraints();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(block1.redstate().get_power(), 0);
    assert!(block1.redstate().is_forced());
    assert_eq!(repeater.redstate().get_power(), 16);
    assert_eq!(block2.redstate().get_power(), 16);
    assert!(block2.redstate().is_forced());
    assert_eq!(dust2.redstate().get_power(), 15);
}

#[test]
fn repeater_locked_by_its_neighbor() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust1 = arena.dust("dust1");
    let dust2 = arena.dust("dust2");
    let dust3 = arena.dust("dust3");
    let dust4 = arena.dust("dust4");
    let throughput = arena.repeater("throughput", 2);
    let locker = arena.repeater("locker", 1);
    let output = arena.dust("output");

    // |
    // ^<+
    // +++
    //  *
    ruststone::link(torch, dust1);
    ruststone::link(dust1, dust2);
    ruststone::link(dust1, dust3);
    ruststone::link(dust3, dust4);

    ruststone::link(dust2, throughput);
    ruststone::link(dust4, locker);

    ruststone::link(throughput, output);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, torch, 2);
    ruststone::add_weighted_edge(dust3, torch, 2);
    ruststone::add_weighted_edge(dust4, torch, 3);

    ruststone::add_weighted_edge(output, throughput, 1);

    ruststone::lock(throughput, locker);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(torch.redstate().is_on());
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(dust2.redstate().get_power(), 14);
    assert_eq!(dust3.redstate().get_power(), 14);
    assert_eq!(dust4.redstate().get_power(), 13);
    assert!(locker.redstate().is_on());
    assert!(throughput.redstate().is_off());
    assert_eq!(output.redstate().get_power(), 0);
}

#[test]
fn repeater_locked_by_its_slower_neighbor() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust1 = arena.dust("dust1");
    let dust2 = arena.dust("dust2");
    let dust3 = arena.dust("dust3");
    let dust4 = arena.dust("dust4");
    let throughput = arena.repeater("throughput", 1);
    let locker = arena.repeater("locker", 2);
    let output = arena.dust("output");

    // |
    // ^<+
    // +++
    //  *
    ruststone::link(torch, dust1);
    ruststone::link(dust1, dust2);
    ruststone::link(dust1, dust3);
    ruststone::link(dust3, dust4);

    ruststone::link(dust2, throughput);
    ruststone::link(dust4, locker);

    ruststone::link(throughput, output);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, torch, 2);
    ruststone::add_weighted_edge(dust3, torch, 2);
    ruststone::add_weighted_edge(dust4, torch, 3);

    ruststone::add_weighted_edge(output, throughput, 1);

    ruststone::lock(throughput, locker);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(torch.redstate().is_on());
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(dust2.redstate().get_power(), 14);
    assert_eq!(dust3.redstate().get_power(), 14);
    assert_eq!(dust4.redstate().get_power(), 13);
    assert!(locker.redstate().is_on());
    assert!(throughput.redstate().is_on());
    assert_eq!(output.redstate().get_power(), 15);
}

#[test]
fn repeater_locked_simultaneously_by_its_neighbors() {
    let arena = RedstoneArena::new();

    let torch = arena.torch("torch");
    let dust1 = arena.dust("dust1");
    let dust2 = arena.dust("dust2");
    let dust3 = arena.dust("dust3");
    let dust4 = arena.dust("dust4");
    let throughput = arena.repeater("throughput", 1);
    let locker = arena.repeater("locker", 1);
    let output = arena.dust("output");

    // |
    // ^<+
    // +++
    //  *
    ruststone::link(torch, dust1);
    ruststone::link(dust1, dust2);
    ruststone::link(dust1, dust3);
    ruststone::link(dust3, dust4);

    ruststone::link(dust2, throughput);
    ruststone::link(dust4, locker);

    ruststone::link(throughput, output);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, torch, 2);
    ruststone::add_weighted_edge(dust3, torch, 2);
    ruststone::add_weighted_edge(dust4, torch, 3);

    ruststone::add_weighted_edge(output, throughput, 1);

    ruststone::lock(throughput, locker);

    let cg = ConstraintGraph::collect(output);
    cg.solve_constraints();

    assert!(torch.redstate().is_on());
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(dust2.redstate().get_power(), 14);
    assert_eq!(dust3.redstate().get_power(), 14);
    assert_eq!(dust4.redstate().get_power(), 13);
    assert!(locker.redstate().is_on());
    assert!(throughput.redstate().is_off());
    assert_eq!(output.redstate().get_power(), 0);
}
