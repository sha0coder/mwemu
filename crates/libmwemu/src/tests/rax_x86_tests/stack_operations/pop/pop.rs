
use crate::*;
const DATA_ADDR: u64 = 0x7000;

// POP - Pop Value from Stack
// Loads value from RSP into destination, then increments RSP

// Basic POP register (64-bit)
#[test]
fn test_pop_rax() {
    let code = [
        0x50, // PUSH RAX (put value on stack)
        0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00, // MOV RAX, 0 (clear RAX)
        0x58, // POP RAX (restore from stack)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = 0x1234567890ABCDEF;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x1234567890ABCDEF, "RAX restored from stack");
    assert_eq!(emu.regs().rsp, 0x1000, "RSP back to original");
}

// POP different registers
#[test]
fn test_pop_rbx() {
    let code = [
        0x48, 0xc7, 0xc3, 0x11, 0x22, 0x33, 0x44, // MOV RBX, 0x44332211
        0x53, // PUSH RBX
        0x48, 0x31, 0xdb, // XOR RBX, RBX (clear)
        0x5b, // POP RBX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rbx, 0x44332211, "RBX restored");
}

#[test]
fn test_pop_rcx() {
    let code = [0x51, 0x59, 0xf4]; // PUSH RCX, POP RCX, HLT
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rcx = 0xAAAAAAAABBBBBBBB;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rcx, 0xAAAAAAAABBBBBBBB);
}

// POP all general purpose registers
#[test]
fn test_pop_all_gp_registers() {
    let code = [
        0x50, 0x53, 0x51, 0x52, 0x56, 0x57, 0x55, // PUSH all
        0x5d, 0x5f, 0x5e, 0x5a, 0x59, 0x5b, 0x58, // POP all (reverse order)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = 0x1111111111111111;
    emu.regs_mut().rbx = 0x2222222222222222;
    emu.regs_mut().rcx = 0x3333333333333333;
    emu.regs_mut().rdx = 0x4444444444444444;
    emu.regs_mut().rsi = 0x5555555555555555;
    emu.regs_mut().rdi = 0x6666666666666666;
    emu.regs_mut().rbp = 0x7777777777777777;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x7777777777777777, "RBP restored");
    assert_eq!(emu.regs().rdi, 0x6666666666666666, "RDI restored");
    assert_eq!(emu.regs().rsi, 0x5555555555555555, "RSI restored");
    assert_eq!(emu.regs().rdx, 0x4444444444444444, "RDX restored");
    assert_eq!(emu.regs().rcx, 0x3333333333333333, "RCX restored");
    assert_eq!(emu.regs().rbx, 0x2222222222222222, "RBX restored");
    assert_eq!(emu.regs().rax, 0x1111111111111111, "RAX restored");
    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored");
}

// POP extended registers (R8-R15)
#[test]
fn test_pop_r8() {
    let code = [
        0x41, 0x50, // PUSH R8
        0x49, 0x31, 0xc0, // XOR R8, R8
        0x41, 0x58, // POP R8
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().r8 = 0xCCCCCCCCCCCCCCCC;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r8, 0xCCCCCCCCCCCCCCCC);
}

#[test]
fn test_pop_r15() {
    let code = [
        0x41, 0x57, // PUSH R15
        0x41, 0x5f, // POP R15
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().r15 = 0xDDDDDDDDDDDDDDDD;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().r15, 0xDDDDDDDDDDDDDDDD);
}

// POP increments RSP
#[test]
fn test_pop_increments_rsp() {
    let code = [
        0x6a, 0x42, // PUSH 0x42
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x1000, "RSP back to original after PUSH/POP");
    assert_eq!(emu.regs().rax, 0x42, "Value popped");
}

// Multiple POP operations
#[test]
fn test_multiple_pop() {
    let code = [
        0x6a, 0x11, // PUSH 0x11
        0x6a, 0x22, // PUSH 0x22
        0x6a, 0x33, // PUSH 0x33
        0x58, // POP RAX (gets 0x33)
        0x5b, // POP RBX (gets 0x22)
        0x59, // POP RCX (gets 0x11)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0x33, "Last pushed, first popped");
    assert_eq!(emu.regs().rbx, 0x22, "Middle value");
    assert_eq!(emu.regs().rcx, 0x11, "First pushed, last popped");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack balanced");
}

// POP preserves flags
#[test]
fn test_pop_preserves_flags() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x48, 0x83, 0xc0, 0x01, // ADD RAX, 1 (sets ZF)
        0x50, // PUSH RAX
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert!(emu.flags().dump() & 0x40 != 0, "ZF should still be set");
}

// POP RSP (special case - uses value from stack)
#[test]
fn test_pop_rsp() {
    let code = [
        0x54, // PUSH RSP
        0x5c, // POP RSP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored from stack");
}

// Test LIFO (Last In First Out) behavior
#[test]
fn test_lifo_behavior() {
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0xc7, 0xc3, 0x02, 0x00, 0x00, 0x00, // MOV RBX, 2
        0x48, 0xc7, 0xc1, 0x03, 0x00, 0x00, 0x00, // MOV RCX, 3
        0x50, 0x53, 0x51, // PUSH RAX, RBX, RCX
        0x5a, 0x5f, 0x5e, // POP RDX, RDI, RSI
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rdx, 3, "RDX gets last pushed (RCX=3)");
    assert_eq!(emu.regs().rdi, 2, "RDI gets middle (RBX=2)");
    assert_eq!(emu.regs().rsi, 1, "RSI gets first pushed (RAX=1)");
}

// POP with zero value
#[test]
fn test_pop_zero() {
    let code = [
        0x6a, 0x00, // PUSH 0
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, -1
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0, "RAX should be 0");
}

// POP with maximum value
#[test]
fn test_pop_max_value() {
    let code = [
        0x48, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff, // MOV RAX, 0xFFFFFFFF
        0x50, // PUSH RAX
        0x48, 0x31, 0xc0, // XOR RAX, RAX
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    // MOV r64, imm32 sign-extends: 0xFFFFFFFF becomes 0xFFFFFFFFFFFFFFFF
    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX should be sign-extended max");
}

// Practical use case: function epilogue
#[test]
fn test_pop_practical_function_epilogue() {
    let code = [
        // Prologue
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x50, // PUSH RAX
        0x53, // PUSH RBX
        // ... function body would be here ...
        // Epilogue
        0x5b, // POP RBX (restore RBX)
        0x58, // POP RAX (restore RAX)
        0x5d, // POP RBP (restore RBP)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.regs_mut().rax = 0x1111;
    emu.regs_mut().rbx = 0x2222;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rbp, 0x2000, "RBP restored");
    assert_eq!(emu.regs().rax, 0x1111, "RAX restored");
    assert_eq!(emu.regs().rbx, 0x2222, "RBX restored");
    assert_eq!(emu.regs().rsp, 0x1000, "RSP balanced");
}

// POP from different stack positions
#[test]
fn test_pop_after_stack_manipulation() {
    let code = [
        0x6a, 0x01, // PUSH 1
        0x6a, 0x02, // PUSH 2
        0x6a, 0x03, // PUSH 3
        0x48, 0x83, 0xc4, 0x08, // ADD RSP, 8 (skip one value)
        0x58, // POP RAX (gets 2, not 3)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 2, "Skipped top value, got second");
}

// Test with all extended registers
#[test]
fn test_pop_all_extended_regs() {
    let code = [
        0x41, 0x50, 0x41, 0x51, 0x41, 0x52, 0x41, 0x53, // PUSH R8-R11
        0x41, 0x54, 0x41, 0x55, 0x41, 0x56, 0x41, 0x57, // PUSH R12-R15
        0x41, 0x5f, 0x41, 0x5e, 0x41, 0x5d, 0x41, 0x5c, // POP R15-R12
        0x41, 0x5b, 0x41, 0x5a, 0x41, 0x59, 0x41, 0x58, // POP R11-R8
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().r8 = 0x08;
    emu.regs_mut().r9 = 0x09;
    emu.regs_mut().r10 = 0x0A;
    emu.regs_mut().r11 = 0x0B;
    emu.regs_mut().r12 = 0x0C;
    emu.regs_mut().r13 = 0x0D;
    emu.regs_mut().r14 = 0x0E;
    emu.regs_mut().r15 = 0x0F;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r15, 0x0F, "R15 restored");
    assert_eq!(emu.regs().r14, 0x0E, "R14 restored");
    assert_eq!(emu.regs().r13, 0x0D, "R13 restored");
    assert_eq!(emu.regs().r12, 0x0C, "R12 restored");
    assert_eq!(emu.regs().r11, 0x0B, "R11 restored");
    assert_eq!(emu.regs().r10, 0x0A, "R10 restored");
    assert_eq!(emu.regs().r9, 0x09, "R9 restored");
    assert_eq!(emu.regs().r8, 0x08, "R8 restored");
}

// Chain of PUSHes and POPs
#[test]
fn test_push_pop_chain() {
    let code = [
        0x48, 0xc7, 0xc0, 0x11, 0x00, 0x00, 0x00, // MOV RAX, 0x11
        0x50, // PUSH RAX
        0x48, 0xc7, 0xc0, 0x22, 0x00, 0x00, 0x00, // MOV RAX, 0x22
        0x50, // PUSH RAX
        0x48, 0xc7, 0xc0, 0x33, 0x00, 0x00, 0x00, // MOV RAX, 0x33
        0x50, // PUSH RAX
        0x5b, // POP RBX (0x33)
        0x59, // POP RCX (0x22)
        0x5a, // POP RDX (0x11)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rbx, 0x33);
    assert_eq!(emu.regs().rcx, 0x22);
    assert_eq!(emu.regs().rdx, 0x11);
}

// Test stack alignment
#[test]
fn test_stack_alignment() {
    let code = [
        0x50, 0x50, 0x50, 0x50, // PUSH RAX 4 times
        0x58, 0x58, 0x58, 0x58, // POP RAX 4 times
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = 0x42;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x1000, "Stack aligned after equal PUSH/POP");
}

// POP into same register multiple times
#[test]
fn test_pop_same_register_multiple() {
    let code = [
        0x6a, 0x01, // PUSH 1
        0x6a, 0x02, // PUSH 2
        0x6a, 0x03, // PUSH 3
        0x58, // POP RAX (gets 3)
        0x58, // POP RAX (gets 2, overwrites 3)
        0x58, // POP RAX (gets 1, overwrites 2)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 1, "RAX has last popped value");
}

// Test interaction with MOV
#[test]
fn test_pop_with_mov() {
    let code = [
        0x48, 0xc7, 0xc0, 0xaa, 0x00, 0x00, 0x00, // MOV RAX, 0xAA
        0x50, // PUSH RAX
        0x48, 0xc7, 0xc0, 0xbb, 0x00, 0x00, 0x00, // MOV RAX, 0xBB
        0x48, 0x89, 0xc3, // MOV RBX, RAX
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rax, 0xAA, "RAX restored from stack");
    assert_eq!(emu.regs().rbx, 0xBB, "RBX has copied value");
}

// Deep stack test
#[test]
fn test_deep_stack() {
    let code = [
        0x6a, 0x01, 0x6a, 0x02, 0x6a, 0x03, 0x6a, 0x04, 0x6a, 0x05,
        0x6a, 0x06, 0x6a, 0x07, 0x6a, 0x08, 0x6a, 0x09, 0x6a, 0x0a,
        // Pop all back
        0x58, 0x58, 0x58, 0x58, 0x58, 0x58, 0x58, 0x58, 0x58, 0x58,
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    assert_eq!(emu.regs().rsp, 0x1000, "Deep stack balanced");
    assert_eq!(emu.regs().rax, 1, "Last pop gets first push");
}

// Test POP doesn't affect other registers
#[test]
fn test_pop_preserves_other_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x33, 0x00, 0x00, 0x00, // MOV RBX, 0x33
        0x48, 0xc7, 0xc1, 0x44, 0x00, 0x00, 0x00, // MOV RCX, 0x44
        0x6a, 0x99, // PUSH 0x99
        0x58, // POP RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();
    // PUSH imm8 sign-extends: 0x99 (bit 7 set) -> 0xFFFFFFFFFFFFFF99
    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFF99, "RAX popped (sign-extended)");
    assert_eq!(emu.regs().rbx, 0x33, "RBX unchanged");
    assert_eq!(emu.regs().rcx, 0x44, "RCX unchanged");
}
