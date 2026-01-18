use crate::*;

// IDIV — Signed Divide
//
// Opcodes:
// - F6 /7       IDIV r/m8      AL := AX / r/m8; AH := AX % r/m8 (signed)
// - F7 /7       IDIV r/m16     AX := DX:AX / r/m16; DX := DX:AX % r/m16 (signed)
// - F7 /7       IDIV r/m32     EAX := EDX:EAX / r/m32; EDX := EDX:EAX % r/m32 (signed)
// - REX.W+F7 /7 IDIV r/m64     RAX := RDX:RAX / r/m64; RDX := RDX:RAX % r/m64 (signed)
//
// Operation: dividend / divisor = quotient, remainder (all signed)
//
// Flags: Undefined (not set by IDIV)
//
// Exceptions:
// - #DE (Divide Error): if divisor is 0 or quotient doesn't fit
//
// CRITICAL: IDIV works with SIGNED integers (two's complement).
// The dividend must be sign-extended into the upper register:
// - For 8-bit:  sign-extend AL into AH (use CBW instruction)
// - For 16-bit: sign-extend AX into DX (use CWD instruction)
// - For 32-bit: sign-extend EAX into EDX (use CDQ instruction)
// - For 64-bit: sign-extend RAX into RDX (use CQO instruction)

// ============================================================================
// 8-bit IDIV (opcode F6 /7)
// ============================================================================

#[test]
fn test_idiv_al_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 100 / 10 = 10 remainder 0 (both positive)
    let code = [
        0x66, 0x98, // CBW (sign-extend AL to AX) - needs 0x66 in 64-bit mode
        0xf6, 0xfb, // IDIV BL (F6 /7)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 100;  // AL = 100
    emu.regs_mut().rbx = 10;   // BL = 10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL (quotient) = 100 / 10 = 10");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH (remainder) = 0");
}

#[test]
fn test_idiv_al_negative_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -100 / 10 = -10 remainder 0
    // -100 in two's complement (i8) = 0x9C
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    emu.regs_mut().rax = (-100i32) as u64 & 0xFF; // AL = 0x9C (-100 in i8)
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = (emu.regs().rax & 0xFF) as i8;
    assert_eq!(quotient, -10, "-100 / 10 = -10");
}

#[test]
fn test_idiv_al_negative_divisor() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 100 / -10 = -10 remainder 0
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    emu.regs_mut().rax = 100;
    emu.regs_mut().rbx = (-10i32) as u64 & 0xFF; // BL = 0xF6 (-10 in i8)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = (emu.regs().rax & 0xFF) as i8;
    assert_eq!(quotient, -10, "100 / -10 = -10");
}

#[test]
fn test_idiv_al_both_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -100 / -10 = 10 remainder 0
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    emu.regs_mut().rax = (-100i32) as u64 & 0xFF;
    emu.regs_mut().rbx = (-10i32) as u64 & 0xFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = (emu.regs().rax & 0xFF) as i8;
    assert_eq!(quotient, 10, "-100 / -10 = 10");
}

#[test]
fn test_idiv_al_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 100 / 7 = 14 remainder 2
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    emu.regs_mut().rax = 100;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 14, "AL (quotient)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 2, "AH (remainder)");
}

#[test]
fn test_idiv_al_negative_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -100 / 7 = -14 remainder -2 (in two's complement, remainder has same sign as dividend)
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xfb, // IDIV BL
        0xf4,
    ];
    emu.regs_mut().rax = (-100i32) as u64 & 0xFF;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = (emu.regs().rax & 0xFF) as i8;
    let remainder = ((emu.regs().rax >> 8) & 0xFF) as i8;
    assert_eq!(quotient, -14, "Quotient");
    assert_eq!(remainder, -2, "Remainder");
}

// ============================================================================
// 16-bit IDIV (opcode F7 /7 with 0x66 prefix)
// ============================================================================

#[test]
fn test_idiv_ax_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000 / 10 = 100 remainder 0
    let code = [
        0x66, 0x99,     // CWD (sign-extend AX to DX:AX) - needs 0x66 in 64-bit mode
        0x66, 0xf7, 0xfb, // IDIV BX (66 F7 /7)
        0xf4,
    ];
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 100, "AX (quotient)");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX (remainder)");
}

#[test]
fn test_idiv_ax_negative_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000 / 10 = -100 remainder 0
    let code = [
        0x66, 0x99,     // CWD (needs 0x66 in 64-bit mode)
        0x66, 0xf7, 0xfb, // IDIV BX
        0xf4,
    ];
    emu.regs_mut().rax = (-1000i32) as u64 & 0xFFFF;
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = (emu.regs().rax & 0xFFFF) as i16;
    assert_eq!(quotient, -100, "-1000 / 10 = -100");
}

#[test]
fn test_idiv_ax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000 / 7 = 142 remainder 6
    let code = [
        0x66, 0x99,     // CWD (needs 0x66 in 64-bit mode)
        0x66, 0xf7, 0xfb, // IDIV BX
        0xf4,
    ];
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 142, "AX (quotient)");
    assert_eq!(emu.regs().rdx & 0xFFFF, 6, "DX (remainder)");
}

// ============================================================================
// 32-bit IDIV (opcode F7 /7)
// ============================================================================

#[test]
fn test_idiv_eax_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000 / 1000 = 1000 remainder 0
    // CDQ (0x99) sign-extends EAX into EDX (in 32-bit context)
    let code = [
        0x99,       // CDQ (sign-extend EAX to EDX:EAX in 32-bit mode)
        0xf7, 0xfb, // IDIV EBX (F7 /7)
        0xf4,
    ];
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rbx = 1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000, "EAX (quotient)");
    assert_eq!(emu.regs().rdx, 0, "EDX (remainder)");
}

#[test]
fn test_idiv_eax_negative_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000 / 1000 = -1000 remainder 0
    let code = [
        0x99,       // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    emu.regs_mut().rax = (-1000000i32) as u64;
    emu.regs_mut().rbx = 1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax as i32;
    assert_eq!(quotient, -1000, "-1000000 / 1000 = -1000");
}

#[test]
fn test_idiv_eax_negative_divisor() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000 / -1000 = -1000 remainder 0
    let code = [
        0x99,       // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rbx = (-1000i32) as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax as i32;
    assert_eq!(quotient, -1000, "1000000 / -1000 = -1000");
}

#[test]
fn test_idiv_eax_both_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000 / -1000 = 1000 remainder 0
    let code = [
        0x99,       // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    emu.regs_mut().rax = (-1000000i32) as u64;
    emu.regs_mut().rbx = (-1000i32) as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax as i32;
    assert_eq!(quotient, 1000, "-1000000 / -1000 = 1000");
}

#[test]
fn test_idiv_eax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000 / 7 = 142857 remainder 1
    let code = [
        0x99,       // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 142857, "EAX (quotient)");
    assert_eq!(emu.regs().rdx, 1, "EDX (remainder)");
}

#[test]
fn test_idiv_eax_negative_dividend_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000 / 7 = -142857 remainder -1
    let code = [
        0x99,       // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    emu.regs_mut().rax = (-1000000i32) as u64;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax as i32;
    let remainder = emu.regs().rdx as i32;
    assert_eq!(quotient, -142857, "EAX (quotient)");
    assert_eq!(remainder, -1, "EDX (remainder)");
}

// ============================================================================
// 64-bit IDIV (opcode REX.W + F7 /7)
// ============================================================================

#[test]
fn test_idiv_rax_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000000000 / 1000000 = 1000000 remainder 0
    // CQO (0x48 0x99) sign-extends RAX into RDX
    let code = [
        0x48, 0x99,    // CQO (sign-extend RAX to RDX:RAX)
        0x48, 0xf7, 0xfb, // IDIV RBX (REX.W F7 /7)
        0xf4,
    ];
    emu.regs_mut().rax = 1000000000000;
    emu.regs_mut().rbx = 1000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000000, "RAX (quotient)");
    assert_eq!(emu.regs().rdx, 0, "RDX (remainder)");
}

#[test]
fn test_idiv_rax_negative_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000000000 / 1000000 = -1000000 remainder 0
    let code = [
        0x48, 0x99,    // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    emu.regs_mut().rax = (-1000000000000i64) as u64;
    emu.regs_mut().rbx = 1000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax as i64;
    assert_eq!(quotient, -1000000, "-1000000000000 / 1000000 = -1000000");
}

#[test]
fn test_idiv_rax_negative_divisor() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000000000 / -1000000 = -1000000 remainder 0
    let code = [
        0x48, 0x99,    // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    emu.regs_mut().rax = 1000000000000;
    emu.regs_mut().rbx = (-1000000i64) as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax as i64;
    assert_eq!(quotient, -1000000, "1000000000000 / -1000000 = -1000000");
}

#[test]
fn test_idiv_rax_both_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000000000 / -1000000 = 1000000 remainder 0
    let code = [
        0x48, 0x99,    // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    emu.regs_mut().rax = (-1000000000000i64) as u64;
    emu.regs_mut().rbx = (-1000000i64) as u64;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax as i64;
    assert_eq!(quotient, 1000000, "-1000000000000 / -1000000 = 1000000");
}

#[test]
fn test_idiv_rax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000000000 / 7 = 142857142857 remainder 1
    let code = [
        0x48, 0x99,    // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    emu.regs_mut().rax = 1000000000000;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 142857142857, "RAX (quotient)");
    assert_eq!(emu.regs().rdx, 1, "RDX (remainder)");
}

#[test]
fn test_idiv_rax_negative_dividend_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -1000000000000 / 7 = -142857142857 remainder -1
    let code = [
        0x48, 0x99,    // CQO
        0x48, 0xf7, 0xfb, // IDIV RBX
        0xf4,
    ];
    emu.regs_mut().rax = (-1000000000000i64) as u64;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax as i64;
    let remainder = emu.regs().rdx as i64;
    assert_eq!(quotient, -142857142857, "RAX (quotient)");
    assert_eq!(remainder, -1, "RDX (remainder)");
}

// ============================================================================
// Different registers
// ============================================================================

#[test]
fn test_idiv_cl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // IDIV CL (8-bit)
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0xf9, // IDIV CL
        0xf4,
    ];
    emu.regs_mut().rax = 100;
    emu.regs_mut().rcx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL (quotient)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH (remainder)");
}

#[test]
fn test_idiv_ecx_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // IDIV ECX (32-bit)
    let code = [
        0x99,       // CDQ
        0xf7, 0xf9, // IDIV ECX
        0xf4,
    ];
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rcx = 1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000, "EAX (quotient)");
    assert_eq!(emu.regs().rdx, 0, "EDX (remainder)");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_idiv_r8b() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0x41, 0xf6, 0xf8, // IDIV R8B
        0xf4,
    ];
    emu.regs_mut().rax = 100;
    emu.regs_mut().r8 = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL (quotient)");
}

#[test]
fn test_idiv_r10d() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x99,       // CDQ
        0x41, 0xf7, 0xfa, // IDIV R10D
        0xf4,
    ];
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().r10 = 1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000, "EAX (quotient)");
}

#[test]
fn test_idiv_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x99,    // CQO
        0x49, 0xf7, 0xff, // IDIV R15
        0xf4,
    ];
    emu.regs_mut().rax = 1000000000000;
    emu.regs_mut().r15 = 1000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000000, "RAX (quotient)");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_idiv_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x98, // CBW (needs 0x66 in 64-bit mode)
        0xf6, 0x3d, 0xf8, 0x0f, 0x00, 0x00, // IDIV BYTE PTR [rip+0x0FF8] (DATA_ADDR=0x2000, RIP after=0x1008)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 10);

    emu.regs_mut().rax = 100;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL (quotient)");
}

#[test]
fn test_idiv_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x99,       // CDQ
        0xf7, 0x3d, 0xf9, 0x0f, 0x00, 0x00, // IDIV DWORD PTR [rip+0x0FF9] (DATA_ADDR=0x2000, RIP after=0x1007)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 1000);

    emu.regs_mut().rax = 1000000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000, "EAX (quotient)");
    assert_eq!(emu.regs().rdx, 0, "EDX (remainder)");
}

#[test]
fn test_idiv_qword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0x99,    // CQO
        0x48, 0xf7, 0x3d, 0xf7, 0x0f, 0x00, 0x00, // IDIV QWORD PTR [rip+0x0FF7] (DATA_ADDR=0x2000, RIP after=0x1009)
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 1000000);

    emu.regs_mut().rax = 1000000000000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000000, "RAX (quotient)");
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_idiv_small_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 5 / 10 = 0 remainder 5
    let code = [
        0x99,       // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    emu.regs_mut().rax = 5;
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "Quotient = 0");
    assert_eq!(emu.regs().rdx, 5, "Remainder = 5");
}

#[test]
fn test_idiv_negative_small_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // -5 / 10 = 0 remainder -5
    let code = [
        0x99,       // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    emu.regs_mut().rax = (-5i32) as u64;
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let quotient = emu.regs().rax as i32;
    let remainder = emu.regs().rdx as i32;
    assert_eq!(quotient, 0, "Quotient = 0");
    assert_eq!(remainder, -5, "Remainder = -5 (sign of dividend)");
}

#[test]
fn test_idiv_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1024 / 256 = 4 remainder 0
    let code = [
        0x99,       // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    emu.regs_mut().rax = 1024;
    emu.regs_mut().rbx = 256;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 4, "Quotient");
    assert_eq!(emu.regs().rdx, 0, "Remainder");
}

#[test]
fn test_idiv_max_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x7FFFFFFF / 1 = 0x7FFFFFFF remainder 0 (max i32)
    let code = [
        0x99,       // CDQ
        0xf7, 0xfb, // IDIV EBX
        0xf4,
    ];
    emu.regs_mut().rax = 0x7FFFFFFF;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x7FFFFFFF, "Quotient = max i32");
    assert_eq!(emu.regs().rdx, 0, "Remainder = 0");
}
