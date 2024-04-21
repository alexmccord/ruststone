use ruststone::{RedstoneGraph, RedstoneArena};

#[test]
fn torch_and_dust() {
    let arena = RedstoneArena::new();

    let torch = arena.make_torch("torch");
    let dust = arena.make_dust("dust");

    torch.link(dust);

    ruststone::add_weighted_edge(dust, torch, 1);

    let rg = RedstoneGraph::collect(torch);
    rg.run();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust.redstate().get_power(), 15);
}

#[test]
fn torch_and_dust_and_dust_and_dust() {
    let arena = RedstoneArena::new();

    let torch = arena.make_torch("torch");
    let dust1 = arena.make_dust("dust1");
    let dust2 = arena.make_dust("dust2");
    let dust3 = arena.make_dust("dust3");

    torch.link(dust1);
    dust1.link(dust2);
    dust2.link(dust3);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, torch, 2);
    ruststone::add_weighted_edge(dust3, torch, 3);

    let rg = RedstoneGraph::collect(torch);
    rg.run();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(dust2.redstate().get_power(), 14);
    assert_eq!(dust3.redstate().get_power(), 13);
}

#[test]
fn torch_and_dust_until_it_runs_out_of_redpower() {
    let arena = RedstoneArena::new();

    let torch = arena.make_torch("torch");
    let dust1 = arena.make_dust("dust1");
    let dust2 = arena.make_dust("dust2");
    let dust3 = arena.make_dust("dust3");
    let dust4 = arena.make_dust("dust4");
    let dust5 = arena.make_dust("dust5");
    let dust6 = arena.make_dust("dust6");
    let dust7 = arena.make_dust("dust7");
    let dust8 = arena.make_dust("dust8");
    let dust9 = arena.make_dust("dust9");
    let dust10 = arena.make_dust("dust10");
    let dust11 = arena.make_dust("dust11");
    let dust12 = arena.make_dust("dust12");
    let dust13 = arena.make_dust("dust13");
    let dust14 = arena.make_dust("dust14");
    let dust15 = arena.make_dust("dust15");
    let dust16 = arena.make_dust("dust16");
    let dust17 = arena.make_dust("dust17");

    torch.link(dust1);
    dust1.link(dust2);
    dust2.link(dust3);
    dust3.link(dust4);
    dust4.link(dust5);
    dust5.link(dust6);
    dust6.link(dust7);
    dust7.link(dust8);
    dust8.link(dust9);
    dust9.link(dust10);
    dust10.link(dust11);
    dust11.link(dust12);
    dust12.link(dust13);
    dust13.link(dust14);
    dust14.link(dust15);
    dust15.link(dust16);
    dust16.link(dust17);

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

    let rg = RedstoneGraph::collect(torch);
    rg.run();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust15.redstate().get_power(), 1);
    assert_eq!(dust16.redstate().get_power(), 0);
    assert_eq!(dust17.redstate().get_power(), 0);
}

#[test]
fn dust_in_the_middle_of_two_torches() {
    let arena = RedstoneArena::new();

    let torch_l = arena.make_torch("torch_l");
    let dust1 = arena.make_dust("dust1");
    let dust2 = arena.make_dust("dust2");
    let dust3 = arena.make_dust("dust3");
    let dust4 = arena.make_dust("dust4");
    let dust5 = arena.make_dust("dust5");
    let torch_r = arena.make_torch("torch_r");

    torch_l.link(dust1);
    dust1.link(dust2);
    dust2.link(dust3);
    dust3.link(dust4);
    dust4.link(dust5);
    torch_r.link(dust5);

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

    let rg = RedstoneGraph::collect(torch_l);
    rg.run();

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

    let torch = arena.make_torch("torch");
    let dust = arena.make_dust("dust");
    let normal_block = arena.make_block("normal_block");
    let output = arena.make_torch("output");

    torch.link(dust);
    dust.link(normal_block);
    normal_block.link(output);

    ruststone::add_weighted_edge(dust, torch, 1);

    let rg = RedstoneGraph::collect(torch);
    rg.run();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust.redstate().get_power(), 15);
    assert_eq!(normal_block.redstate().get_power(), 0);
    assert!(normal_block.redstate().is_forced());
    assert_eq!(output.redstate().get_power(), 0);
}

#[test]
fn torch_and_dust_and_block_and_dust() {
    let arena = RedstoneArena::new();

    let torch = arena.make_torch("torch");
    let dust1 = arena.make_dust("dust1");
    let normal_block = arena.make_block("normal_block");
    let dust2 = arena.make_dust("dust2");

    torch.link(dust1);
    dust1.link(normal_block);
    normal_block.link(dust2);

    ruststone::add_weighted_edge(dust1, torch, 1);

    let rg = RedstoneGraph::collect(torch);
    rg.run();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(normal_block.redstate().get_power(), 0);
    assert!(normal_block.redstate().is_forced());
    assert_eq!(dust2.redstate().get_power(), 0);
}

#[test]
fn and_gate() {
    let arena = RedstoneArena::new();

    let input_l = arena.make_torch("input_l");
    let input_r = arena.make_torch("input_r");
    let and_l = arena.make_torch("and_l");
    let and_r = arena.make_torch("and_r");
    let output = arena.make_torch("output");

    let dust_l = arena.make_dust("dust_l");
    let dust_m = arena.make_dust("dust_m");
    let dust_r = arena.make_dust("dust_r");

    let block_l = arena.make_block("block_l");
    let block_m = arena.make_block("block_m");
    let block_r = arena.make_block("block_r");

    input_l.link(dust_l);
    dust_l.link(block_l);
    block_l.link(and_l);

    input_r.link(dust_r);
    dust_r.link(block_r);
    block_r.link(and_r);

    and_l.link(dust_m);
    and_r.link(dust_m);
    dust_m.link(block_m);

    block_m.link(output);

    ruststone::add_weighted_edge(dust_l, input_l, 1);
    ruststone::add_weighted_edge(dust_r, input_r, 1);
    ruststone::add_weighted_edge(dust_m, and_l, 1);
    ruststone::add_weighted_edge(dust_m, and_r, 1);

    let rg = RedstoneGraph::collect(output);
    rg.run();

    assert!(input_l.redstate().is_on());
    assert!(input_r.redstate().is_on());
    assert!(and_l.redstate().is_off());
    assert!(and_r.redstate().is_off());
    assert!(output.redstate().is_on());
}

#[test]
fn and_gate_with_one_arm_off() {
    let arena = RedstoneArena::new();

    let input_r = arena.make_torch("input_r");
    let and_l = arena.make_torch("and_l");
    let and_r = arena.make_torch("and_r");
    let output = arena.make_torch("output");

    let dust_l = arena.make_dust("dust_l");
    let dust_m = arena.make_dust("dust_m");
    let dust_r = arena.make_dust("dust_r");

    let block_l = arena.make_block("block_l");
    let block_m = arena.make_block("block_m");
    let block_r = arena.make_block("block_r");

    dust_l.link(block_l);
    block_l.link(and_l);

    input_r.link(dust_r);
    dust_r.link(block_r);
    block_r.link(and_r);

    and_l.link(dust_m);
    and_r.link(dust_m);
    dust_m.link(block_m);

    block_m.link(output);

    ruststone::add_weighted_edge(dust_r, input_r, 1);
    ruststone::add_weighted_edge(dust_m, and_l, 1);
    ruststone::add_weighted_edge(dust_m, and_r, 1);

    let rg = RedstoneGraph::collect(output);
    rg.run();

    assert!(input_r.redstate().is_on());
    assert!(and_l.redstate().is_on());
    assert!(and_r.redstate().is_off());
    assert!(output.redstate().is_off());
}

#[test]
fn and_gate_with_both_arms_off() {
    let arena = RedstoneArena::new();

    let and_l = arena.make_torch("and_l");
    let and_r = arena.make_torch("and_r");
    let output = arena.make_torch("output");

    let dust_l = arena.make_dust("dust_l");
    let dust_m = arena.make_dust("dust_m");
    let dust_r = arena.make_dust("dust_r");

    let block_l = arena.make_block("block_l");
    let block_m = arena.make_block("block_m");
    let block_r = arena.make_block("block_r");

    dust_l.link(block_l);
    block_l.link(and_l);

    dust_r.link(block_r);
    block_r.link(and_r);

    and_l.link(dust_m);
    and_r.link(dust_m);
    dust_m.link(block_m);

    block_m.link(output);

    ruststone::add_weighted_edge(dust_m, and_l, 1);
    ruststone::add_weighted_edge(dust_m, and_r, 1);

    let rg = RedstoneGraph::collect(output);
    rg.run();

    assert!(and_l.redstate().is_on());
    assert!(and_r.redstate().is_on());
    assert!(output.redstate().is_off());
}

#[test]
fn xor_gate() {
    let arena = RedstoneArena::new();

    let input_l = arena.make_torch("input_l");
    let input_dust_l = arena.make_dust("input_dust_l");
    let dust_block_l = arena.make_block("dust_block_l");
    let torch_on_top_block_l = arena.make_torch("torch_on_top_block_l");
    let torch_in_front_block_l = arena.make_torch("torch_in_front_block_l");
    let dust_after_inversion_l = arena.make_dust("dust_after_inversion_l");
    let dust_after_inversion_l2 = arena.make_dust("dust_after_inversion_l2");
    let block_after_inversion_l = arena.make_block("block_after_inversion_l");
    let torch_after_dust_inversion_l = arena.make_torch("torch_after_dust_inversion_l");

    input_l.link(input_dust_l);
    input_dust_l.link(dust_block_l);
    dust_block_l.link(torch_on_top_block_l);
    dust_block_l.link(torch_in_front_block_l);
    torch_in_front_block_l.link(dust_after_inversion_l);
    dust_after_inversion_l.link(dust_after_inversion_l2);
    dust_after_inversion_l2.link(block_after_inversion_l);
    block_after_inversion_l.link(torch_after_dust_inversion_l);

    ruststone::add_weighted_edge(input_dust_l, input_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, torch_in_front_block_l, 2);

    let input_r = arena.make_torch("input_r");
    let input_dust_r = arena.make_dust("input_dust_r");
    let dust_block_r = arena.make_block("dust_block_r");
    let torch_on_top_block_r = arena.make_torch("torch_on_top_block_r");
    let torch_in_front_block_r = arena.make_torch("torch_in_front_block_r");
    let dust_after_inversion_r = arena.make_dust("dust_after_inversion_r");
    let dust_after_inversion_r2 = arena.make_dust("dust_after_inversion_r2");
    let block_after_inversion_r = arena.make_block("block_after_inversion_r");
    let torch_after_dust_inversion_r = arena.make_torch("torch_after_dust_inversion_r");

    input_r.link(input_dust_r);
    input_dust_r.link(dust_block_r);
    dust_block_r.link(torch_on_top_block_r);
    dust_block_r.link(torch_in_front_block_r);
    torch_in_front_block_r.link(dust_after_inversion_r);
    dust_after_inversion_r.link(dust_after_inversion_r2);
    dust_after_inversion_r2.link(block_after_inversion_r);
    block_after_inversion_r.link(torch_after_dust_inversion_r);

    ruststone::add_weighted_edge(input_dust_r, input_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, torch_in_front_block_r, 2);

    let and_dust_1 = arena.make_dust("and_dust_1");
    let and_dust_2 = arena.make_dust("and_dust_2");
    let and_block = arena.make_block("and_block");
    let inversion_of_and = arena.make_torch("inversion_of_and");

    torch_on_top_block_l.link(and_dust_1);
    torch_on_top_block_r.link(and_dust_1);
    and_dust_1.link(and_dust_2);
    and_dust_2.link(and_block);
    and_block.link(inversion_of_and);

    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_l, 2);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_r, 2);

    inversion_of_and.link(dust_after_inversion_l);
    inversion_of_and.link(dust_after_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_l, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, inversion_of_and, 2);
    ruststone::add_weighted_edge(dust_after_inversion_r, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, inversion_of_and, 2);

    let output = arena.make_dust("output");

    torch_after_dust_inversion_l.link(output);
    torch_after_dust_inversion_r.link(output);

    ruststone::add_weighted_edge(output, torch_after_dust_inversion_l, 1);
    ruststone::add_weighted_edge(output, torch_after_dust_inversion_r, 1);

    let rg = RedstoneGraph::collect(output);
    rg.run();

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

    let input_dust_l = arena.make_dust("input_dust_l");
    let dust_block_l = arena.make_block("dust_block_l");
    let torch_on_top_block_l = arena.make_torch("torch_on_top_block_l");
    let torch_in_front_block_l = arena.make_torch("torch_in_front_block_l");
    let dust_after_inversion_l = arena.make_dust("dust_after_inversion_l");
    let dust_after_inversion_l2 = arena.make_dust("dust_after_inversion_l2");
    let block_after_inversion_l = arena.make_block("block_after_inversion_l");
    let torch_after_dust_inversion_l = arena.make_torch("torch_after_dust_inversion_l");

    input_dust_l.link(dust_block_l);
    dust_block_l.link(torch_on_top_block_l);
    dust_block_l.link(torch_in_front_block_l);
    torch_in_front_block_l.link(dust_after_inversion_l);
    dust_after_inversion_l.link(dust_after_inversion_l2);
    dust_after_inversion_l2.link(block_after_inversion_l);
    block_after_inversion_l.link(torch_after_dust_inversion_l);

    ruststone::add_weighted_edge(dust_after_inversion_l, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, torch_in_front_block_l, 2);

    let input_r = arena.make_torch("input_r");
    let input_dust_r = arena.make_dust("input_dust_r");
    let dust_block_r = arena.make_block("dust_block_r");
    let torch_on_top_block_r = arena.make_torch("torch_on_top_block_r");
    let torch_in_front_block_r = arena.make_torch("torch_in_front_block_r");
    let dust_after_inversion_r = arena.make_dust("dust_after_inversion_r");
    let dust_after_inversion_r2 = arena.make_dust("dust_after_inversion_r2");
    let block_after_inversion_r = arena.make_block("block_after_inversion_r");
    let torch_after_dust_inversion_r = arena.make_torch("torch_after_dust_inversion_r");

    input_r.link(input_dust_r);
    input_dust_r.link(dust_block_r);
    dust_block_r.link(torch_on_top_block_r);
    dust_block_r.link(torch_in_front_block_r);
    torch_in_front_block_r.link(dust_after_inversion_r);
    dust_after_inversion_r.link(dust_after_inversion_r2);
    dust_after_inversion_r2.link(block_after_inversion_r);
    block_after_inversion_r.link(torch_after_dust_inversion_r);

    ruststone::add_weighted_edge(input_dust_r, input_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, torch_in_front_block_r, 2);

    let and_dust_1 = arena.make_dust("and_dust_1");
    let and_dust_2 = arena.make_dust("and_dust_2");
    let and_block = arena.make_block("and_block");
    let inversion_of_and = arena.make_torch("inversion_of_and");

    torch_on_top_block_l.link(and_dust_1);
    torch_on_top_block_r.link(and_dust_1);
    and_dust_1.link(and_dust_2);
    and_dust_2.link(and_block);
    and_block.link(inversion_of_and);

    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_l, 2);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_r, 2);

    inversion_of_and.link(dust_after_inversion_l);
    inversion_of_and.link(dust_after_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_l, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, inversion_of_and, 2);
    ruststone::add_weighted_edge(dust_after_inversion_r, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, inversion_of_and, 2);

    let output = arena.make_dust("output");

    torch_after_dust_inversion_l.link(output);
    torch_after_dust_inversion_r.link(output);

    ruststone::add_weighted_edge(output, torch_after_dust_inversion_l, 1);
    ruststone::add_weighted_edge(output, torch_after_dust_inversion_r, 1);

    let rg = RedstoneGraph::collect(output);
    rg.run();

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

    let input_l = arena.make_torch("input_l");
    let input_dust_l = arena.make_dust("input_dust_l");
    let dust_block_l = arena.make_block("dust_block_l");
    let torch_on_top_block_l = arena.make_torch("torch_on_top_block_l");
    let torch_in_front_block_l = arena.make_torch("torch_in_front_block_l");
    let dust_after_inversion_l = arena.make_dust("dust_after_inversion_l");
    let dust_after_inversion_l2 = arena.make_dust("dust_after_inversion_l2");
    let block_after_inversion_l = arena.make_block("block_after_inversion_l");
    let torch_after_dust_inversion_l = arena.make_torch("torch_after_dust_inversion_l");

    input_l.link(input_dust_l);
    input_dust_l.link(dust_block_l);
    dust_block_l.link(torch_on_top_block_l);
    dust_block_l.link(torch_in_front_block_l);
    torch_in_front_block_l.link(dust_after_inversion_l);
    dust_after_inversion_l.link(dust_after_inversion_l2);
    dust_after_inversion_l2.link(block_after_inversion_l);
    block_after_inversion_l.link(torch_after_dust_inversion_l);

    ruststone::add_weighted_edge(input_dust_l, input_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, torch_in_front_block_l, 2);

    let input_dust_r = arena.make_dust("input_dust_r");
    let dust_block_r = arena.make_block("dust_block_r");
    let torch_on_top_block_r = arena.make_torch("torch_on_top_block_r");
    let torch_in_front_block_r = arena.make_torch("torch_in_front_block_r");
    let dust_after_inversion_r = arena.make_dust("dust_after_inversion_r");
    let dust_after_inversion_r2 = arena.make_dust("dust_after_inversion_r2");
    let block_after_inversion_r = arena.make_block("block_after_inversion_r");
    let torch_after_dust_inversion_r = arena.make_torch("torch_after_dust_inversion_r");

    input_dust_r.link(dust_block_r);
    dust_block_r.link(torch_on_top_block_r);
    dust_block_r.link(torch_in_front_block_r);
    torch_in_front_block_r.link(dust_after_inversion_r);
    dust_after_inversion_r.link(dust_after_inversion_r2);
    dust_after_inversion_r2.link(block_after_inversion_r);
    block_after_inversion_r.link(torch_after_dust_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_r, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, torch_in_front_block_r, 2);

    let and_dust_1 = arena.make_dust("and_dust_1");
    let and_dust_2 = arena.make_dust("and_dust_2");
    let and_block = arena.make_block("and_block");
    let inversion_of_and = arena.make_torch("inversion_of_and");

    torch_on_top_block_l.link(and_dust_1);
    torch_on_top_block_r.link(and_dust_1);
    and_dust_1.link(and_dust_2);
    and_dust_2.link(and_block);
    and_block.link(inversion_of_and);

    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_l, 2);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_r, 2);

    inversion_of_and.link(dust_after_inversion_l);
    inversion_of_and.link(dust_after_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_l, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, inversion_of_and, 2);
    ruststone::add_weighted_edge(dust_after_inversion_r, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, inversion_of_and, 2);

    let output = arena.make_dust("output");

    torch_after_dust_inversion_l.link(output);
    torch_after_dust_inversion_r.link(output);

    ruststone::add_weighted_edge(output, torch_after_dust_inversion_l, 1);
    ruststone::add_weighted_edge(output, torch_after_dust_inversion_r, 1);

    let rg = RedstoneGraph::collect(output);
    rg.run();

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

    let input_dust_l = arena.make_dust("input_dust_l");
    let dust_block_l = arena.make_block("dust_block_l");
    let torch_on_top_block_l = arena.make_torch("torch_on_top_block_l");
    let torch_in_front_block_l = arena.make_torch("torch_in_front_block_l");
    let dust_after_inversion_l = arena.make_dust("dust_after_inversion_l");
    let dust_after_inversion_l2 = arena.make_dust("dust_after_inversion_l2");
    let block_after_inversion_l = arena.make_block("block_after_inversion_l");
    let torch_after_dust_inversion_l = arena.make_torch("torch_after_dust_inversion_l");

    input_dust_l.link(dust_block_l);
    dust_block_l.link(torch_on_top_block_l);
    dust_block_l.link(torch_in_front_block_l);
    torch_in_front_block_l.link(dust_after_inversion_l);
    dust_after_inversion_l.link(dust_after_inversion_l2);
    dust_after_inversion_l2.link(block_after_inversion_l);
    block_after_inversion_l.link(torch_after_dust_inversion_l);

    ruststone::add_weighted_edge(dust_after_inversion_l, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, torch_in_front_block_l, 2);

    let input_dust_r = arena.make_dust("input_dust_r");
    let dust_block_r = arena.make_block("dust_block_r");
    let torch_on_top_block_r = arena.make_torch("torch_on_top_block_r");
    let torch_in_front_block_r = arena.make_torch("torch_in_front_block_r");
    let dust_after_inversion_r = arena.make_dust("dust_after_inversion_r");
    let dust_after_inversion_r2 = arena.make_dust("dust_after_inversion_r2");
    let block_after_inversion_r = arena.make_block("block_after_inversion_r");
    let torch_after_dust_inversion_r = arena.make_torch("torch_after_dust_inversion_r");

    input_dust_r.link(dust_block_r);
    dust_block_r.link(torch_on_top_block_r);
    dust_block_r.link(torch_in_front_block_r);
    torch_in_front_block_r.link(dust_after_inversion_r);
    dust_after_inversion_r.link(dust_after_inversion_r2);
    dust_after_inversion_r2.link(block_after_inversion_r);
    block_after_inversion_r.link(torch_after_dust_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_r, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, torch_in_front_block_r, 2);

    let and_dust_1 = arena.make_dust("and_dust_1");
    let and_dust_2 = arena.make_dust("and_dust_2");
    let and_block = arena.make_block("and_block");
    let inversion_of_and = arena.make_torch("inversion_of_and");

    torch_on_top_block_l.link(and_dust_1);
    torch_on_top_block_r.link(and_dust_1);
    and_dust_1.link(and_dust_2);
    and_dust_2.link(and_block);
    and_block.link(inversion_of_and);

    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_l, 1);
    ruststone::add_weighted_edge(and_dust_1, torch_in_front_block_r, 1);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_l, 2);
    ruststone::add_weighted_edge(and_dust_2, torch_in_front_block_r, 2);

    inversion_of_and.link(dust_after_inversion_l);
    inversion_of_and.link(dust_after_inversion_r);

    ruststone::add_weighted_edge(dust_after_inversion_l, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_l2, inversion_of_and, 2);
    ruststone::add_weighted_edge(dust_after_inversion_r, inversion_of_and, 1);
    ruststone::add_weighted_edge(dust_after_inversion_r2, inversion_of_and, 2);

    let output = arena.make_dust("output");

    torch_after_dust_inversion_l.link(output);
    torch_after_dust_inversion_r.link(output);

    ruststone::add_weighted_edge(output, torch_after_dust_inversion_l, 1);
    ruststone::add_weighted_edge(output, torch_after_dust_inversion_r, 1);

    let rg = RedstoneGraph::collect(output);
    rg.run();

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

    let block_a = arena.make_block("block_a");
    let torch_a = arena.make_torch("torch_a");
    let dust_a1 = arena.make_dust("dust_a1");
    let dust_a2 = arena.make_dust("dust_a2");

    let block_b = arena.make_block("block_b");
    let torch_b = arena.make_torch("torch_b");
    let dust_b1 = arena.make_dust("dust_b1");
    let dust_b2 = arena.make_dust("dust_b2");

    block_a.link(torch_a);
    torch_a.link(dust_a1);
    dust_a1.link(dust_a2);
    dust_a2.link(block_b);

    ruststone::add_weighted_edge(dust_a1, torch_a, 1);
    ruststone::add_weighted_edge(dust_a2, torch_a, 2);

    block_b.link(torch_b);
    torch_b.link(dust_b1);
    dust_b1.link(dust_b2);
    dust_b2.link(block_a);

    ruststone::add_weighted_edge(dust_b1, torch_b, 1);
    ruststone::add_weighted_edge(dust_b2, torch_b, 2);

    let rg = RedstoneGraph::collect(block_a);
    rg.run();

    assert!(torch_a.redstate().is_on());
    assert!(torch_b.redstate().is_off());
}

#[test]
fn memory_cell_alt() {
    let arena = RedstoneArena::new();

    let block_a = arena.make_block("block_a");
    let torch_a = arena.make_torch("torch_a");
    let dust_a1 = arena.make_dust("dust_a1");
    let dust_a2 = arena.make_dust("dust_a2");

    let block_b = arena.make_block("block_b");
    let torch_b = arena.make_torch("torch_b");
    let dust_b1 = arena.make_dust("dust_b1");
    let dust_b2 = arena.make_dust("dust_b2");

    block_a.link(torch_a);
    torch_a.link(dust_a1);
    dust_a1.link(dust_a2);
    dust_a2.link(block_b);

    ruststone::add_weighted_edge(dust_a1, torch_a, 1);
    ruststone::add_weighted_edge(dust_a2, torch_a, 2);

    block_b.link(torch_b);
    torch_b.link(dust_b1);
    dust_b1.link(dust_b2);
    dust_b2.link(block_a);

    ruststone::add_weighted_edge(dust_b1, torch_b, 1);
    ruststone::add_weighted_edge(dust_b2, torch_b, 2);

    let rg = RedstoneGraph::collect(block_b);
    rg.run();

    assert!(torch_a.redstate().is_off());
    assert!(torch_b.redstate().is_on());
}

#[test]
fn torch_and_dust_and_block_and_repeater() {
    let arena = RedstoneArena::new();

    let torch = arena.make_torch("torch");
    let dust = arena.make_dust("dust");
    let block = arena.make_block("block");
    let repeater = arena.make_repeater("repeater", 1);

    torch.link(dust);
    dust.link(block);
    block.link(repeater);

    ruststone::add_weighted_edge(dust, torch, 1);

    let rg = RedstoneGraph::collect(torch);
    rg.run();

    assert_eq!(torch.redstate().get_power(), 16);
    assert_eq!(dust.redstate().get_power(), 15);
    assert_eq!(block.redstate().get_power(), 0);
    assert!(block.redstate().is_forced());
    assert_eq!(repeater.redstate().get_power(), 16);
}

#[test]
fn torch_and_dust_and_block_and_repeater_and_block_and_dust() {
    let arena = RedstoneArena::new();

    let torch = arena.make_torch("torch");
    let dust1 = arena.make_dust("dust1");
    let block1 = arena.make_block("block1");
    let repeater = arena.make_repeater("repeater", 1);
    let block2 = arena.make_block("block2");
    let dust2 = arena.make_dust("dust2");

    torch.link(dust1);
    dust1.link(block1);
    block1.link(repeater);
    repeater.link(block2);
    block2.link(dust2);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, block2, 1);

    let rg = RedstoneGraph::collect(torch);
    rg.run();

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

    let torch = arena.make_torch("torch");
    let dust1 = arena.make_dust("dust1");
    let dust2 = arena.make_dust("dust2");
    let dust3 = arena.make_dust("dust3");
    let dust4 = arena.make_dust("dust4");
    let throughput = arena.make_repeater("throughput", 2);
    let locker = arena.make_repeater("locker", 1);
    let output = arena.make_dust("output");

    // |
    // ^<+
    // +++
    //  *
    torch.link(dust1);
    dust1.link(dust2);
    dust1.link(dust3);
    dust3.link(dust4);

    dust2.link(throughput);
    dust4.link(locker);

    throughput.link(output);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, torch, 2);
    ruststone::add_weighted_edge(dust3, torch, 2);
    ruststone::add_weighted_edge(dust4, torch, 3);

    ruststone::add_weighted_edge(output, throughput, 1);

    ruststone::lock(throughput, locker);

    let rg = RedstoneGraph::collect(output);
    rg.run();

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

    let torch = arena.make_torch("torch");
    let dust1 = arena.make_dust("dust1");
    let dust2 = arena.make_dust("dust2");
    let dust3 = arena.make_dust("dust3");
    let dust4 = arena.make_dust("dust4");
    let throughput = arena.make_repeater("throughput", 1);
    let locker = arena.make_repeater("locker", 2);
    let output = arena.make_dust("output");

    // |
    // ^<+
    // +++
    //  *
    torch.link(dust1);
    dust1.link(dust2);
    dust1.link(dust3);
    dust3.link(dust4);

    dust2.link(throughput);
    dust4.link(locker);

    throughput.link(output);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, torch, 2);
    ruststone::add_weighted_edge(dust3, torch, 2);
    ruststone::add_weighted_edge(dust4, torch, 3);

    ruststone::add_weighted_edge(output, throughput, 1);

    ruststone::lock(throughput, locker);

    let rg = RedstoneGraph::collect(output);
    rg.run();

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

    let torch = arena.make_torch("torch");
    let dust1 = arena.make_dust("dust1");
    let dust2 = arena.make_dust("dust2");
    let dust3 = arena.make_dust("dust3");
    let dust4 = arena.make_dust("dust4");
    let throughput = arena.make_repeater("throughput", 1);
    let locker = arena.make_repeater("locker", 1);
    let output = arena.make_dust("output");

    // |
    // ^<+
    // +++
    //  *
    torch.link(dust1);
    dust1.link(dust2);
    dust1.link(dust3);
    dust3.link(dust4);

    dust2.link(throughput);
    dust4.link(locker);

    throughput.link(output);

    ruststone::add_weighted_edge(dust1, torch, 1);
    ruststone::add_weighted_edge(dust2, torch, 2);
    ruststone::add_weighted_edge(dust3, torch, 2);
    ruststone::add_weighted_edge(dust4, torch, 3);

    ruststone::add_weighted_edge(output, throughput, 1);

    ruststone::lock(throughput, locker);

    let rg = RedstoneGraph::collect(output);
    rg.run();

    assert!(torch.redstate().is_on());
    assert_eq!(dust1.redstate().get_power(), 15);
    assert_eq!(dust2.redstate().get_power(), 14);
    assert_eq!(dust3.redstate().get_power(), 14);
    assert_eq!(dust4.redstate().get_power(), 13);
    assert!(locker.redstate().is_on());
    assert!(throughput.redstate().is_off());
    assert_eq!(output.redstate().get_power(), 0);
}
