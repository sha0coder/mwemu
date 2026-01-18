
use crate::*;
use std::convert::TryInto;
const DATA_ADDR: u64 = 0x7000;

// PUSH - Push Value onto Stack
// Decrements RSP and stores value at new RSP location

// Basic PUSH register (64-bit)
#[test]
fn test_push_rax() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x22, 0x33, 0x44, // MOV RAX, 0x44332211
        0x50, // PUSH RAX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x0FF8, "RSP should be decremented by 8");

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    let pushed_val = u64::from_le_bytes(stack_val);
    assert_eq!(pushed_val, 0x44332211, "Pushed value should be on stack");
}

// PUSH different registers
#[test]
fn test_push_rbx() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc3, 0xaa, 0xbb, 0xcc, 0xdd, // MOV RBX, 0xDDCCBBAA
        0x53, // PUSH RBX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x1FF8, "RSP decremented by 8");

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x1FF8, stack_val.len()).try_into().unwrap();
    // MOV r64, imm32 sign-extends: 0xDDCCBBAA (bit 31 set) -> 0xFFFFFFFFDDCCBBAA
    assert_eq!(u64::from_le_bytes(stack_val), 0xFFFFFFFFDDCCBBAA, "RBX value on stack");
}

#[test]
fn test_push_rcx() {
    let mut emu = emu64();
    let code = [0x51, 0xf4]; // PUSH RCX, HLT
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rcx = 0x1234567890ABCDEF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x0FF8, "RSP decremented");

    let mut stack_val = [0u8; 8];
    stack_val = emu.maps.read_bytes(0x0FF8, stack_val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(stack_val), 0x1234567890ABCDEF);
}

// PUSH all general purpose registers
#[test]
fn test_push_all_gp_registers() {
    let mut emu = emu64();
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        0x52, // PUSH RDX
        0x56, // PUSH RSI
        0x57, // PUSH RDI
        0x55, // PUSH RBP
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = 0x1111111111111111;
    emu.regs_mut().rbx = 0x2222222222222222;
    emu.regs_mut().rcx = 0x3333333333333333;
    emu.regs_mut().rdx = 0x4444444444444444;
    emu.regs_mut().rsi = 0x5555555555555555;
    emu.regs_mut().rdi = 0x6666666666666666;
    emu.regs_mut().rbp = 0x7777777777777777;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x1000 - 7 * 8, "RSP decremented by 7*8 = 56");

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x1000 - 8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x1111111111111111, "RAX");
    val = emu.maps.read_bytes(0x1000 - 16, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x2222222222222222, "RBX");
    val = emu.maps.read_bytes(0x1000 - 24, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x3333333333333333, "RCX");
}

// PUSH extended registers (R8-R15)
#[test]
fn test_push_r8() {
    let mut emu = emu64();
    let code = [0x41, 0x50, 0xf4]; // PUSH R8, HLT
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().r8 = 0xAAAAAAAAAAAAAAAA;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x0FF8);

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0xAAAAAAAAAAAAAAAA);
}

#[test]
fn test_push_r15() {
    let mut emu = emu64();
    let code = [0x41, 0x57, 0xf4]; // PUSH R15, HLT
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().r15 = 0xBBBBBBBBBBBBBBBB;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x0FF8);

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0xBBBBBBBBBBBBBBBB);
}

// PUSH immediate values
#[test]
fn test_push_imm8() {
    let mut emu = emu64();
    let code = [0x6a, 0x42, 0xf4]; // PUSH 0x42 (sign-extended), HLT
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x0FF8);

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x42);
}

#[test]
fn test_push_imm8_negative() {
    let mut emu = emu64();
    let code = [0x6a, 0xff, 0xf4]; // PUSH -1 (0xFF sign-extended), HLT
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x0FF8);

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn test_push_imm32() {
    let mut emu = emu64();
    let code = [0x68, 0x78, 0x56, 0x34, 0x12, 0xf4]; // PUSH 0x12345678, HLT
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x0FF8);

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x12345678);
}

// PUSH with zero value
#[test]
fn test_push_zero() {
    let mut emu = emu64();
    let code = [0x6a, 0x00, 0xf4]; // PUSH 0, HLT
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0);
}

// Multiple PUSH operations
#[test]
fn test_multiple_push() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x48, 0xc7, 0xc3, 0x22, 0x00, 0x00, 0x00, // MOV RBX, 0x22
        0x48, 0xc7, 0xc1, 0x33, 0x00, 0x00, 0x00, // MOV RCX, 0x33
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0x51, // PUSH RCX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x1000 - 24, "RSP decremented by 3*8");

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x1000 - 8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x11, "First push (RAX)");
    val = emu.maps.read_bytes(0x1000 - 16, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x22, "Second push (RBX)");
    val = emu.maps.read_bytes(0x1000 - 24, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x33, "Third push (RCX)");
}

// PUSH preserves flags
#[test]
fn test_push_preserves_flags() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x50, // PUSH RAX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert!(emu.flags().dump() & 0x40 != 0, "ZF should still be set");
}

// PUSH RSP (special case - pushes value before decrement)
#[test]
fn test_push_rsp() {
    let mut emu = emu64();
    let code = [0x54, 0xf4]; // PUSH RSP, HLT
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x0FF8, "RSP decremented");

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x1000, "Original RSP value pushed");
}

// Test stack grows downward
#[test]
fn test_stack_grows_down() {
    let mut emu = emu64();
    let code = [
        0x6a, 0x01, // PUSH 1
        0x6a, 0x02, // PUSH 2
        0x6a, 0x03, // PUSH 3
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x1000 - 24, "Stack pointer decreased");

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(emu.regs().rsp, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 3, "Top of stack is 3");
}

// PUSH with maximum value
#[test]
fn test_push_max_value() {
    let mut emu = emu64();
    let code = [0x50, 0xf4]; // PUSH RAX, HLT
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0xFFFFFFFFFFFFFFFF);
}

// PUSH followed by modification doesn't affect stack
#[test]
fn test_push_then_modify() {
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x42, 0x00, 0x00, 0x00, // MOV RAX, 0x42
        0x50, // PUSH RAX
        0x48, 0xc7, 0xc0, 0x99, 0x00, 0x00, 0x00, // MOV RAX, 0x99
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x99, "RAX modified");

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x42, "Stack has original value");
}

// Practical use case: function prologue
#[test]
fn test_push_practical_function_prologue() {
    let mut emu = emu64();
    let code = [
        0x55, // PUSH RBP (save old base pointer)
        0x48, 0x89, 0xe5, // MOV RBP, RSP (set new base pointer)
        0x50, // PUSH RAX (save RAX)
        0x53, // PUSH RBX (save RBX)
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.regs_mut().rax = 0x1111;
    emu.regs_mut().rbx = 0x2222;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rbp, 0x0FF8, "RBP set to RSP after first push");
    assert_eq!(emu.regs().rsp, 0x0FF8 - 16, "RSP after pushing RAX and RBX");

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(0x0FF8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x2000, "Old RBP saved");
}

// Test with small stack space
#[test]
fn test_push_near_stack_bottom() {
    let mut emu = emu64();
    let code = [
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x10;
    emu.regs_mut().rax = 0xAAAA;
    emu.regs_mut().rbx = 0xBBBB;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x10 - 16, "RSP at near-zero address");
}

// Chain of pushes and verify order
#[test]
fn test_push_chain_order() {
    let mut emu = emu64();
    let code = [
        0x6a, 0x0a, // PUSH 10
        0x6a, 0x14, // PUSH 20
        0x6a, 0x1e, // PUSH 30
        0x6a, 0x28, // PUSH 40
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(emu.regs().rsp, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 40, "Top of stack");
    val = emu.maps.read_bytes(emu.regs().rsp + 8, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 30);
    val = emu.maps.read_bytes(emu.regs().rsp + 16, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 20);
    val = emu.maps.read_bytes(emu.regs().rsp + 24, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 10, "Bottom of our pushes");
}

// PUSH with all extended registers
#[test]
fn test_push_all_extended_regs() {
    let mut emu = emu64();
    let code = [
        0x41, 0x50, // PUSH R8
        0x41, 0x51, // PUSH R9
        0x41, 0x52, // PUSH R10
        0x41, 0x53, // PUSH R11
        0x41, 0x54, // PUSH R12
        0x41, 0x55, // PUSH R13
        0x41, 0x56, // PUSH R14
        0x41, 0x57, // PUSH R15
        0xf4, // HLT
    ];
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().r8 = 0x08;
    emu.regs_mut().r9 = 0x09;
    emu.regs_mut().r10 = 0x0A;
    emu.regs_mut().r11 = 0x0B;
    emu.regs_mut().r12 = 0x0C;
    emu.regs_mut().r13 = 0x0D;
    emu.regs_mut().r14 = 0x0E;
    emu.regs_mut().r15 = 0x0F;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x1000 - 64, "8 registers * 8 bytes");

    let mut val = [0u8; 8];
    val = emu.maps.read_bytes(emu.regs().rsp, val.len()).try_into().unwrap();
    assert_eq!(u64::from_le_bytes(val), 0x0F, "R15 on top");
}
