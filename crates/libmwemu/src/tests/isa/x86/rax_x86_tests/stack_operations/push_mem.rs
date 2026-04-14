use crate::*;
const DATA_ADDR: u64 = 0x1000;

// Comprehensive tests for PUSH with memory operands
//
// PUSH m64 - Push quadword from memory onto stack
// Various addressing modes:
// - Direct: PUSH [addr]
// - Register indirect: PUSH [reg]
// - Register + displacement: PUSH [reg + disp]
// - Base + index: PUSH [base + index]
// - Base + index + displacement: PUSH [base + index + disp]
// - RIP-relative: PUSH [RIP + disp]

// ============================================================================
// PUSH with register indirect addressing [reg]
// ============================================================================

#[test]
fn test_push_mem_indirect_rax() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x1234567890ABCDEF);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x0FF8, "RSP decremented");
    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x1234567890ABCDEF, "Value from [RAX] pushed");
}

#[test]
fn test_push_mem_indirect_rbx() {
    let code = [
        0xff, 0x33, // PUSH [RBX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbx = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0xDEADBEEFCAFEBABE);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xDEADBEEFCAFEBABE, "Value from [RBX] pushed");
}

#[test]
fn test_push_mem_indirect_rcx() {
    let code = [
        0xff, 0x31, // PUSH [RCX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rcx = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0xAAAAAAAABBBBBBBB);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xAAAAAAAABBBBBBBB, "Value from [RCX] pushed");
}

#[test]
fn test_push_mem_indirect_r8() {
    let code = [
        0x41, 0xff, 0x30, // PUSH [R8]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().r8 = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x1111222233334444);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x1111222233334444, "Value from [R8] pushed");
}

// ============================================================================
// PUSH with displacement [reg + disp8]
// ============================================================================

#[test]
fn test_push_mem_disp8_positive() {
    let code = [
        0xff, 0x70, 0x08, // PUSH [RAX + 8]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR + 8, 0x4242424242424242);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x4242424242424242, "Value from [RAX+8] pushed");
}

#[test]
fn test_push_mem_disp8_negative() {
    let code = [
        0xff, 0x70, 0xf8, // PUSH [RAX - 8]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR + 16;

    emu.maps.write_qword(DATA_ADDR + 8, 0x9999999999999999);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x9999999999999999, "Value from [RAX-8] pushed");
}

#[test]
fn test_push_mem_disp8_zero() {
    let code = [
        0xff, 0x70, 0x00, // PUSH [RAX + 0]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x5555555555555555);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x5555555555555555, "Value from [RAX+0] pushed");
}

// ============================================================================
// PUSH with 32-bit displacement [reg + disp32]
// ============================================================================

#[test]
fn test_push_mem_disp32_large() {
    let code = [
        0xff, 0xb0, 0x00, 0x10, 0x00, 0x00, // PUSH [RAX + 0x1000]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x2000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR + 0x1000, 0x7777777777777777);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x7777777777777777, "Value from [RAX+0x1000] pushed");
}

#[test]
fn test_push_mem_disp32_small() {
    let code = [
        0xff, 0xb0, 0x10, 0x00, 0x00, 0x00, // PUSH [RAX + 0x10]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR + 0x10, 0x1234123412341234);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x1234123412341234, "Value from [RAX+0x10] pushed");
}

// ============================================================================
// PUSH with SIB addressing [base + index]
// ============================================================================

#[test]
fn test_push_mem_sib_base_index() {
    let code = [
        0xff, 0x34, 0x18, // PUSH [RAX + RBX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rbx = 0x100;

    emu.maps.write_qword(DATA_ADDR + 0x100, 0x8888888888888888);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x8888888888888888, "Value from [RAX+RBX] pushed");
}

#[test]
fn test_push_mem_sib_base_index_scale2() {
    let code = [
        0xff, 0x34, 0x58, // PUSH [RAX + RBX*2]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rbx = 0x10;

    emu.maps.write_qword(DATA_ADDR + 0x20, 0x2222222222222222);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x2222222222222222, "Value from [RAX+RBX*2] pushed");
}

#[test]
fn test_push_mem_sib_base_index_scale4() {
    let code = [
        0xff, 0x34, 0x98, // PUSH [RAX + RBX*4]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rbx = 0x08;

    emu.maps.write_qword(DATA_ADDR + 0x20, 0x3333333333333333);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x3333333333333333, "Value from [RAX+RBX*4] pushed");
}

#[test]
fn test_push_mem_sib_base_index_scale8() {
    let code = [
        0xff, 0x34, 0xd8, // PUSH [RAX + RBX*8]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rbx = 0x04;

    emu.maps.write_qword(DATA_ADDR + 0x20, 0x4444444444444444);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x4444444444444444, "Value from [RAX+RBX*8] pushed");
}

// ============================================================================
// PUSH with SIB + displacement [base + index*scale + disp]
// ============================================================================

#[test]
fn test_push_mem_sib_disp8() {
    let code = [
        0xff, 0x74, 0x18, 0x10, // PUSH [RAX + RBX + 0x10]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rbx = 0x20;

    emu.maps.write_qword(DATA_ADDR + 0x30, 0x6666666666666666);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x6666666666666666, "Value from [RAX+RBX+0x10] pushed");
}

#[test]
fn test_push_mem_sib_scale_disp8() {
    let code = [
        0xff, 0x74, 0x58, 0x08, // PUSH [RAX + RBX*2 + 8]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rbx = 0x10;

    emu.maps.write_qword(DATA_ADDR + 0x28, 0xABCDABCDABCDABCD);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xABCDABCDABCDABCD, "Value from [RAX+RBX*2+8] pushed");
}

#[test]
fn test_push_mem_sib_disp32() {
    let code = [
        0xff, 0xb4, 0x18, 0x00, 0x01, 0x00, 0x00, // PUSH [RAX + RBX + 0x100]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rbx = 0x50;

    emu.maps.write_qword(DATA_ADDR + 0x150, 0xFEDCFEDCFEDCFEDC);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFEDCFEDCFEDCFEDC, "Value from [RAX+RBX+0x100] pushed");
}

// ============================================================================
// PUSH multiple memory values
// ============================================================================

#[test]
fn test_push_mem_sequence() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xff, 0x33, // PUSH [RBX]
        0xff, 0x31, // PUSH [RCX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rbx = DATA_ADDR + 0x10;
    emu.regs_mut().rcx = DATA_ADDR + 0x20;

    emu.maps.write_qword(DATA_ADDR, 0x1111111111111111);
    emu.maps.write_qword(DATA_ADDR + 0x10, 0x2222222222222222);
    emu.maps.write_qword(DATA_ADDR + 0x20, 0x3333333333333333);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rsp, 0x1000 - 24, "Three values pushed");
    assert_eq!(emu.maps.read_qword(0x1000 - 8).unwrap(), 0x1111111111111111);
    assert_eq!(
        emu.maps.read_qword(0x1000 - 16).unwrap(),
        0x2222222222222222
    );
    assert_eq!(
        emu.maps.read_qword(0x1000 - 24).unwrap(),
        0x3333333333333333
    );
}

#[test]
fn test_push_mem_array_elements() {
    let code = [
        // Push array[0], array[1], array[2]
        0xff, 0x30, // PUSH [RAX] (array[0])
        0xff, 0x70, 0x08, // PUSH [RAX + 8] (array[1])
        0xff, 0x70, 0x10, // PUSH [RAX + 16] (array[2])
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0xAA);
    emu.maps.write_qword(DATA_ADDR + 8, 0xBB);
    emu.maps.write_qword(DATA_ADDR + 16, 0xCC);

    emu.run(None).unwrap();

    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp).unwrap(),
        0xCC,
        "array[2]"
    );
    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp + 8).unwrap(),
        0xBB,
        "array[1]"
    );
    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp + 16).unwrap(),
        0xAA,
        "array[0]"
    );
}

// ============================================================================
// PUSH memory then POP
// ============================================================================

#[test]
fn test_push_mem_pop_roundtrip() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0x5b, // POP RBX
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0xBEEFBEEFBEEFBEEF);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0xBEEFBEEFBEEFBEEF, "Memory value in RBX");
    assert_eq!(emu.regs().rsp, 0x1000, "Stack balanced");
}

// ============================================================================
// PUSH memory preserves flags and other registers
// ============================================================================

#[test]
fn test_push_mem_preserves_registers() {
    let code = [
        0x48, 0xc7, 0xc3, 0x99, 0x00, 0x00, 0x00, // MOV RBX, 0x99
        0x48, 0xc7, 0xc1, 0x88, 0x00, 0x00, 0x00, // MOV RCX, 0x88
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x1234567890ABCDEF);

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx, 0x99, "RBX unchanged");
    assert_eq!(emu.regs().rcx, 0x88, "RCX unchanged");
}

#[test]
fn test_push_mem_preserves_flags() {
    let code = [
        0xf9, // STC (set carry)
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x42);

    emu.run(None).unwrap();

    assert_ne!(emu.flags().dump() & 0x01, 0, "CF preserved");
}

// ============================================================================
// PUSH with different memory values
// ============================================================================

#[test]
fn test_push_mem_zero() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0, "Zero value pushed");
}

#[test]
fn test_push_mem_all_ones() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0xFFFFFFFFFFFFFFFF);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xFFFFFFFFFFFFFFFF, "All ones pushed");
}

#[test]
fn test_push_mem_alternating_bits() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0xAAAAAAAAAAAAAAAA);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xAAAAAAAAAAAAAAAA, "Alternating bits pushed");
}

// ============================================================================
// PUSH from stack memory (reading stack to push)
// ============================================================================

#[test]
fn test_push_mem_from_stack() {
    let code = [
        0x6a, 0x42, // PUSH 0x42 (put value on stack)
        0x48, 0x89, 0xe0, // MOV RAX, RSP (RAX points to stack)
        0xff, 0x30, // PUSH [RAX] (push copy of top of stack)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.run(None).unwrap();

    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp).unwrap(),
        0x42,
        "Duplicate on top"
    );
    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp + 8).unwrap(),
        0x42,
        "Original"
    );
}

// ============================================================================
// PUSH with various base registers
// ============================================================================

#[test]
fn test_push_mem_base_rsi() {
    let code = [
        0xff, 0x36, // PUSH [RSI]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rsi = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x9876543210FEDCBA);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x9876543210FEDCBA, "Value from [RSI] pushed");
}

#[test]
fn test_push_mem_base_rdi() {
    let code = [
        0xff, 0x37, // PUSH [RDI]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rdi = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x1122334455667788);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x1122334455667788, "Value from [RDI] pushed");
}

#[test]
fn test_push_mem_base_rbp() {
    let code = [
        0xff, 0x75, 0x00, // PUSH [RBP + 0]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rbp = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x5A5A5A5A5A5A5A5A);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x5A5A5A5A5A5A5A5A, "Value from [RBP] pushed");
}

// ============================================================================
// PUSH memory in function parameter passing
// ============================================================================

#[test]
fn test_push_mem_struct_fields() {
    let code = [
        // Assuming RAX points to a struct, push its fields
        0xff, 0x30, // PUSH [RAX] (field 0)
        0xff, 0x70, 0x08, // PUSH [RAX + 8] (field 1)
        0xff, 0x70, 0x10, // PUSH [RAX + 16] (field 2)
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0x1111);
    emu.maps.write_qword(DATA_ADDR + 8, 0x2222);
    emu.maps.write_qword(DATA_ADDR + 16, 0x3333);

    emu.run(None).unwrap();

    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp).unwrap(),
        0x3333,
        "Field 2"
    );
    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp + 8).unwrap(),
        0x2222,
        "Field 1"
    );
    assert_eq!(
        emu.maps.read_qword(emu.regs().rsp + 16).unwrap(),
        0x1111,
        "Field 0"
    );
}

// ============================================================================
// PUSH memory edge cases
// ============================================================================

#[test]
fn test_push_mem_high_address() {
    let code = [
        0xff, 0x30, // PUSH [RAX]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    let space = emu.alloc(
        "space",
        100,
        crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
    );
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = space;

    emu.maps.write_qword(space, 0xDEADBEEF);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xDEADBEEF, "Value from high address pushed");
}

#[test]
fn test_push_mem_with_extended_registers() {
    let code = [
        0x41, 0xff, 0x37, // PUSH [R15]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().r15 = DATA_ADDR;

    emu.maps.write_qword(DATA_ADDR, 0xF15F15F15F15F15);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xF15F15F15F15F15, "Value from [R15] pushed");
}

#[test]
fn test_push_mem_indexed_array_access() {
    let code = [
        // PUSH array[RBX] where array base is in RAX
        0xff, 0x34, 0xd8, // PUSH [RAX + RBX*8]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rbx = 5; // array[5]

    emu.maps.write_qword(DATA_ADDR + 40, 0x5555555555555555);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0x5555555555555555, "array[5] pushed");
}

#[test]
fn test_push_mem_complex_addressing() {
    let code = [
        // PUSH [RAX + RCX*4 + 0x20]
        0xff, 0x74, 0x88, 0x20, // PUSH [RAX + RCX*4 + 0x20]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;
    emu.regs_mut().rcx = 0x10;

    emu.maps.write_qword(DATA_ADDR + 0x60, 0xC0FFEEC0FFEEC0FF);

    emu.run(None).unwrap();

    let val = emu.maps.read_qword(0x0FF8).unwrap();
    assert_eq!(val, 0xC0FFEEC0FFEEC0FF, "Complex address value pushed");
}

#[test]
fn test_push_mem_consecutive_locations() {
    let code = [
        0xff, 0x70, 0x00, // PUSH [RAX + 0]
        0xff, 0x70, 0x08, // PUSH [RAX + 8]
        0xff, 0x70, 0x10, // PUSH [RAX + 16]
        0xff, 0x70, 0x18, // PUSH [RAX + 24]
        0xff, 0x70, 0x20, // PUSH [RAX + 32]
        0xf4, // HLT
    ];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.maps
        .create_map(
            "stack_test",
            0x1000 - (0x1000 / 2),
            0x1000,
            crate::maps::mem64::Permission::READ_WRITE_EXECUTE,
        )
        .unwrap();
    emu.regs_mut().rsp = 0x1000;
    emu.regs_mut().rax = DATA_ADDR;

    for i in 0..5 {
        emu.maps
            .write_qword(DATA_ADDR + i * 8, (i + 1) as u64 * 0x1111111111111111);
    }

    emu.run(None).unwrap();

    for i in 0..5 {
        let val = emu.maps.read_qword(emu.regs().rsp + i * 8).unwrap();
        assert_eq!(val, (5 - i) as u64 * 0x1111111111111111, "Value {}", i);
    }
}
