extern crate gb_core;

use gb_core::cpu::*;
use gb_core::utils::ModifyBytes;

#[test]
/// Test joint 16-bit register operations
fn test_reg_16() {
    let mut gb = Cpu::new();

    gb.set_reg_16(Regs16::BC, 0xABCD);
    assert_eq!(gb.b, 0xAB);
    assert_eq!(gb.c, 0xCD);

    let bc = gb.get_reg_16(Regs16::BC);
    assert_eq!(bc.get_high_byte(), 0xAB);
    assert_eq!(bc.get_low_byte(), 0xCD);
}

#[test]
/// Test flag functions
fn test_flags() {
    let mut gb = Cpu::new();

    gb.f = 0;
    gb.set_flag(Flags::Z);
    assert_eq!(gb.f, 0b1000_0000);
    assert!(gb.get_flag(Flags::Z));

    gb.f = 0xF0;
    gb.clear_flag(Flags::N);
    assert_eq!(gb.f, 0b1011_0000);
    assert_eq!(gb.get_flag(Flags::N), false);
}

#[test]
/// Test 8-bit increment
fn test_inc_8() {
    let mut gb = Cpu::new();

    gb.a = 0;
    gb.f = 0;

    // Check that basic increment works
    gb.inc_8(Regs::A);
    assert_eq!(gb.get_reg(Regs::A), 1);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    // Check that C flag is not modified
    // Check that N flag is always false
    gb.set_flag(Flags::C);
    gb.set_flag(Flags::N);

    gb.inc_8(Regs::A);
    assert_eq!(gb.get_reg(Regs::A), 2);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), true);

    // Check that H flag is set properly
    gb.a = 0x0F;
    gb.inc_8(Regs::A);
    assert_eq!(gb.get_reg(Regs::A), 0x10);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), true);
    assert_eq!(gb.get_flag(Flags::C), true);

    // Check that value overflows properly
    // Check that Z flag is set properly
    gb.a = 0xFF;
    gb.inc_8(Regs::A);
    assert_eq!(gb.get_reg(Regs::A), 0);
    assert_eq!(gb.get_flag(Flags::Z), true);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), true);
    assert_eq!(gb.get_flag(Flags::C), true);
}

#[test]
/// Test 16-bit increment
fn test_inc_16() {
    let mut gb = Cpu::new();

    gb.b = 0;
    gb.c = 0;
    gb.f = 0;

    // Check that basic increment works
    // Don't need to test flags - they are not modified
    gb.inc_16(Regs16::BC);
    assert_eq!(gb.get_reg_16(Regs16::BC), 1);

    // Check that value overflows properly
    gb.b = 0xFF;
    gb.c = 0xFF;
    gb.inc_16(Regs16::BC);
    assert_eq!(gb.get_reg_16(Regs16::BC), 0);
}

// TODO: Add dec tests

#[test]
/// Test 8-bit addition
fn test_add_8() {
    let mut gb = Cpu::new();

    gb.a = 0;
    gb.f = 0;

    // Test basic addition functionality
    gb.add_a_d8(1, false);

    assert_eq!(gb.get_reg(Regs::A), 1);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    // Test add with carry
    gb.set_flag(Flags::C);
    gb.add_a_d8(1, true);

    assert_eq!(gb.get_reg(Regs::A), 3);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    // Test H flag
    gb.a = 0x7F;
    gb.add_a_d8(0x7F, false);

    assert_eq!(gb.get_reg(Regs::A), 0xFE);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), true);
    assert_eq!(gb.get_flag(Flags::C), false);

    // Test value overflows
    // Test C flag
    gb.a = 0xFF;
    gb.add_a_d8(0x7F, false);

    assert_eq!(gb.get_reg(Regs::A), 0x7E);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), true);
    assert_eq!(gb.get_flag(Flags::C), true);
}

#[test]
/// Test 16-bit addition
fn test_add_16() {
    let mut gb = Cpu::new();

    gb.b = 0;
    gb.c = 0;
    gb.f = 0;

    // Test basic addition functionality
    gb.add_nn_d16(Regs16::BC, 1);

    assert_eq!(gb.get_reg_16(Regs16::BC), 1);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    // Test H flag
    gb.b = 0x0F;
    gb.c = 0xFF;
    gb.add_nn_d16(Regs16::BC, 0x000F);

    assert_eq!(gb.get_reg_16(Regs16::BC), 0x100E);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), true);
    assert_eq!(gb.get_flag(Flags::C), false);

    // Test value overflows
    // Test C flag
    gb.b = 0x7F;
    gb.c = 0xFF;
    gb.add_nn_d16(Regs16::BC, 0x8001);

    assert_eq!(gb.get_reg_16(Regs16::BC), 0);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), true);
    assert_eq!(gb.get_flag(Flags::C), true);
}

#[test]
/// Test 8-bit subtraction
fn test_sub() {
    let mut gb = Cpu::new();

    gb.a = 10;
    gb.f = 0;

    // Test basic subtraction functionality
    gb.sub_a_d8(1, false);

    assert_eq!(gb.get_reg(Regs::A), 9);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), true);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    // Test sub with carry
    gb.set_flag(Flags::C);
    gb.sub_a_d8(1, true);

    assert_eq!(gb.get_reg(Regs::A), 7);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), true);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    // Test H flag
    gb.a = 0x10;
    gb.sub_a_d8(1, false);

    assert_eq!(gb.get_reg(Regs::A), 0x0F);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), true);
    assert_eq!(gb.get_flag(Flags::H), true);
    assert_eq!(gb.get_flag(Flags::C), false);

    // Test value underflows
    // Test C flag
    gb.a = 0x7F;
    gb.sub_a_d8(0x90, false);

    assert_eq!(gb.get_reg(Regs::A), 0xEF);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), true);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), true);
}

#[test]
/// Tests AND operation
fn test_and() {
    let mut gb = Cpu::new();

    gb.a = 0xFF;
    gb.f = 0;

    gb.and_a_d8(0xAA);
    assert_eq!(gb.get_reg(Regs::A), 0xAA);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), true);
    assert_eq!(gb.get_flag(Flags::C), false);

    gb.and_a_d8(0x55);
    assert_eq!(gb.get_reg(Regs::A), 0);
    assert_eq!(gb.get_flag(Flags::Z), true);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), true);
    assert_eq!(gb.get_flag(Flags::C), false);
}

#[test]
/// Tests OR operation
fn test_or() {
    let mut gb = Cpu::new();

    gb.a = 0;
    gb.f = 0;

    gb.or_a_d8(0xAA);
    assert_eq!(gb.get_reg(Regs::A), 0xAA);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    gb.or_a_d8(0);
    assert_eq!(gb.get_reg(Regs::A), 0xAA);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    gb.a = 0;
    gb.or_a_d8(0);
    assert_eq!(gb.get_reg(Regs::A), 0);
    assert_eq!(gb.get_flag(Flags::Z), true);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);
}

#[test]
/// Tests XOR operation
fn test_xor() {
    let mut gb = Cpu::new();

    gb.a = 0;
    gb.f = 0;

    gb.xor_a_d8(0xAA);
    assert_eq!(gb.get_reg(Regs::A), 0xAA);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    gb.xor_a_d8(0);
    assert_eq!(gb.get_reg(Regs::A), 0xAA);
    assert_eq!(gb.get_flag(Flags::Z), false);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);

    gb.xor_a_d8(0xAA);
    assert_eq!(gb.get_reg(Regs::A), 0);
    assert_eq!(gb.get_flag(Flags::Z), true);
    assert_eq!(gb.get_flag(Flags::N), false);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);
}

#[test]
/// Tests CP operation
fn test_cp() {
    let mut gb = Cpu::new();

    gb.a = 0;
    gb.f = 0;

    gb.cp_a_d8(0);
    assert_eq!(gb.get_reg(Regs::A), 0);
    assert_eq!(gb.get_flag(Flags::Z), true);
    assert_eq!(gb.get_flag(Flags::H), false);
    assert_eq!(gb.get_flag(Flags::C), false);
}

#[test]
/// Test push and pop operations
fn test_stack() {
    let mut gb = Cpu::new();

    gb.push(0xABCD);
    gb.push(0x1234);
    assert_eq!(gb.sp, 0xFFFA);
    assert_eq!(gb.read_ram(0xFFFB), 0x12);
    assert_eq!(gb.read_ram(0xFFFC), 0x34);
    assert_eq!(gb.read_ram(0xFFFD), 0xAB);
    assert_eq!(gb.read_ram(0xFFFE), 0xCD);

    assert_eq!(gb.pop(), 0x1234);
    assert_eq!(gb.sp, 0xFFFC);
    assert_eq!(gb.pop(), 0xABCD);
    assert_eq!(gb.sp, 0xFFFE);
}

#[test]
#[should_panic]
/// Test invalid stack operation
fn test_invalid_stack() {
    // Not much of a test
    let mut gb = Cpu::new();
    gb.pop();
}
