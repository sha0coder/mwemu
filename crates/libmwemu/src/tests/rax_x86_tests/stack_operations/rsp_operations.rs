use crate::*;
const DATA_ADDR: u64 = 0x7000;

// Comprehensive tests for operations that directly modify RSP
//
// These include:
// - ADD/SUB RSP (stack allocation/deallocation)
// - MOV RSP, reg / MOV reg, RSP (stack pointer manipulation)
// - LEA RSP, [RSP + offset] (stack adjustments)
// - XCHG RSP, reg (stack pointer exchange)
// - INC/DEC RSP (rare but possible)

// ============================================================================
// ADD RSP - Deallocate stack space
// ============================================================================

#[test]
fn test_add_rsp_8() {
    let code = [
        0x48, 0x83, 0xc4, 0x08, // ADD RSP, 8
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1008, "RSP incremented by 8");
}

#[test]
fn test_add_rsp_16() {
    let code = [
        0x48, 0x83, 0xc4, 0x10, // ADD RSP, 16
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1010, "RSP incremented by 16");
}

#[test]
fn test_add_rsp_32() {
    let code = [
        0x48, 0x83, 0xc4, 0x20, // ADD RSP, 32
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1020, "RSP incremented by 32");
}

#[test]
fn test_add_rsp_large() {
    let code = [
        0x48, 0x81, 0xc4, 0x00, 0x10, 0x00, 0x00, // ADD RSP, 0x1000
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "RSP incremented by 4096");
}

#[test]
fn test_add_rsp_cleanup_params() {
    let code = [
        // Simulate: 3 params pushed, clean them up
        0x6a, 0x01, 0x6a, 0x02, 0x6a, 0x03, // PUSH 1, 2, 3
        0x48, 0x83, 0xc4, 0x18, // ADD RSP, 24 (3 * 8)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "Stack cleaned up after parameter push");
}

// ============================================================================
// SUB RSP - Allocate stack space
// ============================================================================

#[test]
fn test_sub_rsp_8() {
    let code = [
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FF8, "RSP decremented by 8");
}

#[test]
fn test_sub_rsp_16() {
    let code = [
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FF0, "RSP decremented by 16");
}

#[test]
fn test_sub_rsp_32() {
    let code = [
        0x48, 0x83, 0xec, 0x20, // SUB RSP, 32
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FE0, "RSP decremented by 32");
}

#[test]
fn test_sub_rsp_large_frame() {
    let code = [
        0x48, 0x81, 0xec, 0x00, 0x10, 0x00, 0x00, // SUB RSP, 0x1000
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x10000-(0x10000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x10000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0xF000, "Large stack frame allocated");
}

#[test]
fn test_sub_rsp_for_locals() {
    let code = [
        0x55, // PUSH RBP
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0x48, 0x83, 0xec, 0x20, // SUB RSP, 32 (local vars)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 40, "Frame + locals allocated");
}

// ============================================================================
// MOV RSP, reg - Set stack pointer
// ============================================================================

#[test]
fn test_mov_rsp_rax() {
    let code = [
        0x48, 0x89, 0xc4, // MOV RSP, RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = 0x2000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "RSP set from RAX");
}

#[test]
fn test_mov_rsp_rbx() {
    let code = [
        0x48, 0x89, 0xdc, // MOV RSP, RBX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbx = 0x3000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x3000, "RSP set from RBX");
}

#[test]
fn test_mov_rsp_rbp() {
    let code = [
        0x48, 0x89, 0xec, // MOV RSP, RBP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "RSP set from RBP (like LEAVE part 1)");
}

#[test]
fn test_mov_rsp_r8() {
    let code = [
        0x4c, 0x89, 0xc4, // MOV RSP, R8
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().r8 = 0x4000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x4000, "RSP set from R8");
}

// ============================================================================
// MOV reg, RSP - Save stack pointer
// ============================================================================

#[test]
fn test_mov_rax_rsp() {
    let code = [
        0x48, 0x89, 0xe0, // MOV RAX, RSP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x1000, "RAX gets RSP value");
    assert_eq!(emu.regs().rsp, 0x1000, "RSP unchanged");
}

#[test]
fn test_mov_rbx_rsp() {
    let code = [
        0x48, 0x89, 0xe3, // MOV RBX, RSP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x1000, "RBX gets RSP value");
}

#[test]
fn test_mov_rbp_rsp() {
    let code = [
        0x48, 0x89, 0xe5, // MOV RBP, RSP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbp, 0x1000, "RBP gets RSP value (prologue pattern)");
}

#[test]
fn test_save_restore_rsp() {
    let code = [
        0x48, 0x89, 0xe0, // MOV RAX, RSP (save)
        0x48, 0x83, 0xec, 0x20, // SUB RSP, 32 (allocate)
        0x48, 0x89, 0xc4, // MOV RSP, RAX (restore)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP saved and restored");
}

// ============================================================================
// LEA RSP, [RSP + offset] - Adjust stack pointer
// ============================================================================

#[test]
fn test_lea_rsp_rsp_plus_8() {
    let code = [
        0x48, 0x8d, 0x64, 0x24, 0x08, // LEA RSP, [RSP + 8]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1008, "RSP adjusted by LEA +8");
}

#[test]
fn test_lea_rsp_rsp_minus_8() {
    let code = [
        0x48, 0x8d, 0x64, 0x24, 0xf8, // LEA RSP, [RSP - 8]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FF8, "RSP adjusted by LEA -8");
}

#[test]
fn test_lea_rsp_rsp_plus_16() {
    let code = [
        0x48, 0x8d, 0x64, 0x24, 0x10, // LEA RSP, [RSP + 16]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1010, "RSP adjusted by LEA +16");
}

#[test]
fn test_lea_rsp_rbp_minus_offset() {
    let code = [
        0x48, 0x8d, 0x65, 0xf0, // LEA RSP, [RBP - 16]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = 0x2000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1FF0, "RSP set via LEA from RBP");
}

// ============================================================================
// INC/DEC RSP (unusual but valid)
// ============================================================================

#[test]
fn test_inc_rsp() {
    let code = [
        0x48, 0xff, 0xc4, // INC RSP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1001, "RSP incremented by 1");
}

#[test]
fn test_dec_rsp() {
    let code = [
        0x48, 0xff, 0xcc, // DEC RSP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FFF, "RSP decremented by 1");
}

#[test]
fn test_multiple_inc_rsp() {
    let code = [
        0x48, 0xff, 0xc4, // INC RSP
        0x48, 0xff, 0xc4, // INC RSP
        0x48, 0xff, 0xc4, // INC RSP
        0x48, 0xff, 0xc4, // INC RSP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1004, "RSP incremented 4 times");
}

// ============================================================================
// XCHG RSP, reg - Exchange stack pointer
// ============================================================================

#[test]
fn test_xchg_rsp_rax() {
    let code = [
        0x48, 0x94, // XCHG RSP, RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = 0x2000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "RSP gets RAX value");
    assert_eq!(emu.regs().rax, 0x1000, "RAX gets old RSP value");
}

#[test]
fn test_xchg_rsp_rbx() {
    let code = [
        0x48, 0x87, 0xdc, // XCHG RSP, RBX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbx = 0x3000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x3000, "RSP gets RBX value");
    assert_eq!(emu.regs().rbx, 0x1000, "RBX gets old RSP value");
}

// ============================================================================
// Complex RSP manipulation patterns
// ============================================================================

#[test]
fn test_rsp_alloc_dealloc_cycle() {
    let code = [
        0x48, 0x83, 0xec, 0x20, // SUB RSP, 32 (allocate)
        0x48, 0x83, 0xc4, 0x20, // ADD RSP, 32 (deallocate)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP restored after alloc/dealloc");
}

#[test]
fn test_rsp_save_alloc_restore() {
    let code = [
        0x48, 0x89, 0xe3, // MOV RBX, RSP (save)
        0x48, 0x83, 0xec, 0x40, // SUB RSP, 64
        // Do work...
        0x48, 0x89, 0xdc, // MOV RSP, RBX (restore)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP fully restored");
}

#[test]
fn test_rsp_with_stack_operations() {
    let code = [
        0x48, 0x83, 0xec, 0x10, // SUB RSP, 16
        0x48, 0xc7, 0x04, 0x24, 0x42, 0x00, 0x00, 0x00, // MOV [RSP], 0x42
        0x48, 0x83, 0xc4, 0x10, // ADD RSP, 16
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP balanced");
    let val = emu.maps.read_qword(0x0FF0).unwrap();
    assert_eq!(val, 0x42, "Value written to allocated space");
}

#[test]
fn test_rsp_lea_cleanup() {
    let code = [
        0x6a, 0x01, 0x6a, 0x02, 0x6a, 0x03, // PUSH 3 values
        0x48, 0x8d, 0x64, 0x24, 0x18, // LEA RSP, [RSP + 24] (clean up)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "Stack cleaned up via LEA");
}

// ============================================================================
// RSP arithmetic operations
// ============================================================================

#[test]
fn test_add_rsp_rax() {
    let code = [
        0x48, 0x01, 0xc4, // ADD RSP, RAX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = 0x100;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1100, "RSP += RAX");
}

#[test]
fn test_sub_rsp_rbx() {
    let code = [
        0x48, 0x29, 0xdc, // SUB RSP, RBX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbx = 0x50;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FB0, "RSP -= RBX");
}

#[test]
fn test_and_rsp_align() {
    let code = [
        0x48, 0x83, 0xe4, 0xf0, // AND RSP, -16 (align to 16 bytes)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1008-(0x1008 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1008;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP aligned to 16 bytes");
}

// ============================================================================
// RSP with memory operations
// ============================================================================

#[test]
fn test_mov_rsp_from_memory() {
    let code = [
        0x48, 0x8b, 0x20, // MOV RSP, [RAX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x2000);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x2000, "RSP loaded from memory");
}

#[test]
fn test_mov_memory_from_rsp() {
    let code = [
        0x48, 0x89, 0x20, // MOV [RAX], RSP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1234-(0x1234 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1234;
    emu.regs_mut().rax = DATA_ADDR;
    emu.run(None).unwrap();

    let val = emu.maps.read_qword(DATA_ADDR).unwrap();
    assert_eq!(val, 0x1234, "RSP stored to memory");
}

// ============================================================================
// Edge cases and special scenarios
// ============================================================================

#[test]
fn test_rsp_zero() {
    let code = [
        0x48, 0x31, 0xe4, // XOR RSP, RSP
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0, "RSP can be zeroed");
}

#[test]
fn test_rsp_max_value() {
    let code = [
        0x48, 0xc7, 0xc4, 0xff, 0xff, 0xff, 0xff, // MOV RSP, -1
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0xFFFFFFFFFFFFFFFF, "RSP can be set to max value");
}

#[test]
fn test_rsp_wraparound_add() {
    let code = [
        0x48, 0x83, 0xc4, 0x10, // ADD RSP, 16
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0xFFFFFFFFFFFFFFF0-(0xFFFFFFFFFFFFFFF0 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0xFFFFFFFFFFFFFFF0;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0, "RSP wraps around on overflow");
}

#[test]
fn test_rsp_complex_calculation() {
    let code = [
        0x48, 0x89, 0xe0, // MOV RAX, RSP
        0x48, 0x83, 0xec, 0x20, // SUB RSP, 32
        0x48, 0x01, 0xc4, // ADD RSP, RAX (RSP += old RSP)
        0x48, 0x29, 0xc4, // SUB RSP, RAX (RSP -= old RSP)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    // RSP = 0x1000 - 32 + 0x1000 - 0x1000 = 0x1000 - 32 = 0xFE0
    assert_eq!(emu.regs().rsp, 0x0FE0, "Complex RSP calculation");
}

#[test]
fn test_rsp_in_loop_simulation() {
    let code = [
        // Simulate stack operations in a loop
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0x48, 0x83, 0xc4, 0x08, // ADD RSP, 8
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0x48, 0x83, 0xc4, 0x08, // ADD RSP, 8
        0x48, 0x83, 0xec, 0x08, // SUB RSP, 8
        0x48, 0x83, 0xc4, 0x08, // ADD RSP, 8
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps.create_map("stack_test", 0x1000-(0x1000 / 2), 0x1000, crate::maps::mem64::Permission::READ_WRITE_EXECUTE).unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000, "RSP balanced after loop simulation");
}
