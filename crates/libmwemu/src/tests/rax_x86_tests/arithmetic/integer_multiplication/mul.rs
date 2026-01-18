use crate::*;

// MUL — Unsigned Multiply
//
// Opcodes:
// - F6 /4       MUL r/m8      Unsigned multiply (AX := AL * r/m8)
// - REX + F6 /4 MUL r/m8*     Unsigned multiply (with REX for extended regs)
// - F7 /4       MUL r/m16     Unsigned multiply (DX:AX := AX * r/m16)
// - F7 /4       MUL r/m32     Unsigned multiply (EDX:EAX := EAX * r/m32)
// - REX.W+F7 /4 MUL r/m64     Unsigned multiply (RDX:RAX := RAX * r/m64)
//
// Operation:
//   8-bit:  AX := AL * r/m8
//   16-bit: DX:AX := AX * r/m16
//   32-bit: EDX:EAX := EAX * r/m32
//   64-bit: RDX:RAX := RAX * r/m64
//
// Flags: CF and OF are set to 0 if upper half of result is 0, otherwise 1.
//        SF, ZF, AF, PF are undefined (not tested).
//
// CRITICAL: MUL uses implicit operands (AL/AX/EAX/RAX) and stores results
// in double-width register pairs.

// ============================================================================
// 8-bit MUL (opcode F6 /4) - Result in AX (AH:AL)
// ============================================================================

#[test]
fn test_mul_al_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xe3, // MUL BL (F6 /4, ModRM=11_100_011)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 0x05; // AL = 5
    emu.regs_mut().rbx = 0x03; // BL = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x000F, "5 * 3 = 15 (0x000F in AX)");
    assert!(!emu.flags().f_cf, "CF should be clear (high byte AH is 0)");
    assert!(!emu.flags().f_of, "OF should be clear (high byte AH is 0)");
}

#[test]
fn test_mul_al_overflow_to_ah() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 200 * 3 = 600 = 0x0258, AH=0x02, AL=0x58
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 200; // AL = 200
    emu.regs_mut().rbx = 3;   // BL = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0258, "200 * 3 = 600 = 0x0258");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0x02, "AH should be 0x02");
    assert_eq!(emu.regs().rax & 0xFF, 0x58, "AL should be 0x58");
    assert!(emu.flags().f_cf, "CF should be set (AH is non-zero)");
    assert!(emu.flags().f_of, "OF should be set (AH is non-zero)");
}

#[test]
fn test_mul_al_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0x00; // AL = 0
    emu.regs_mut().rbx = 0xFF; // BL = 255
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "0 * 255 = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_mul_al_by_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0xFF; // AL = 255
    emu.regs_mut().rbx = 0x00; // BL = 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "255 * 0 = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_al_max_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 255 * 255 = 65025 = 0xFE01
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0xFF; // AL = 255
    emu.regs_mut().rbx = 0xFF; // BL = 255
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFE01, "255 * 255 = 65025 = 0xFE01");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0xFE, "AH = 0xFE");
    assert_eq!(emu.regs().rax & 0xFF, 0x01, "AL = 0x01");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_mul_al_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0x42; // AL = 66
    emu.regs_mut().rbx = 0x01; // BL = 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0042, "66 * 1 = 66");
    assert!(!emu.flags().f_cf, "CF should be clear (no overflow to AH)");
}

#[test]
fn test_mul_al_by_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 128 * 2 = 256 = 0x0100 (AH=0x01, AL=0x00)
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 128; // AL = 128
    emu.regs_mut().rbx = 2;   // BL = 2
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0100, "128 * 2 = 256 = 0x0100");
    assert!(emu.flags().f_cf, "CF should be set (overflow to AH)");
}

#[test]
fn test_mul_preserves_upper_rax_bytes() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0xDEADBEEF_CAFEBABE; // Only AL (0xBE) is used
    emu.regs_mut().rbx = 0x02;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // AL=0xBE * BL=0x02 = 0x017C
    assert_eq!(emu.regs().rax & 0xFFFF, 0x017C, "AX contains result");
    assert_eq!(emu.regs().rax & !0xFFFF, 0xDEADBEEF_CAFE0000, "Upper bytes preserved");
}

// ============================================================================
// 16-bit MUL (opcode F7 /4 with 0x66 prefix) - Result in DX:AX
// ============================================================================

#[test]
fn test_mul_ax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0xe3, // MUL BX (66 F7 /4)
        0xf4,
    ];
    emu.regs_mut().rax = 0x0005; // AX = 5
    emu.regs_mut().rbx = 0x0003; // BX = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x000F, "AX: 5 * 3 = 15");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0x0000, "DX should be 0 (no overflow)");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_mul_ax_overflow_to_dx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000 * 100 = 100000 = 0x0186A0
    // DX = 0x0001, AX = 0x86A0
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    emu.regs_mut().rax = 1000; // AX = 1000
    emu.regs_mut().rbx = 100;  // BX = 100
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x86A0, "AX (low word) = 0x86A0");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0x0001, "DX (high word) = 0x0001");
    assert!(emu.flags().f_cf, "CF should be set (overflow to DX)");
    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_mul_ax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    emu.regs_mut().rax = 0x0000;
    emu.regs_mut().rbx = 0xFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "AX = 0");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_ax_max_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 65535 * 65535 = 4294836225 = 0xFFFE0001
    // DX = 0xFFFE, AX = 0x0001
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    emu.regs_mut().rax = 0xFFFF;
    emu.regs_mut().rbx = 0xFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0001, "AX = 0x0001");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0xFFFE, "DX = 0xFFFE");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_mul_ax_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    emu.regs_mut().rax = 0x1234;
    emu.regs_mut().rbx = 0x0001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x1234, "AX unchanged");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_ax_squares() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 256 * 256 = 65536 = 0x010000
    // DX = 0x0001, AX = 0x0000
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    emu.regs_mut().rax = 256;
    emu.regs_mut().rbx = 256;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0000, "AX = 0x0000");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0x0001, "DX = 0x0001");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_mul_cx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xe1, 0xf4]; // MUL CX
    emu.regs_mut().rax = 10;
    emu.regs_mut().rcx = 20;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 200, "10 * 20 = 200");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX = 0");
}

// ============================================================================
// 32-bit MUL (opcode F7 /4, no prefix in 64-bit mode) - Result in EDX:EAX
// ============================================================================

#[test]
fn test_mul_eax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0xe3, // MUL EBX (F7 /4)
        0xf4,
    ];
    emu.regs_mut().rax = 100;
    emu.regs_mut().rbx = 200;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 20000, "EAX: 100 * 200 = 20000");
    assert_eq!(emu.regs().rdx, 0, "EDX should be 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_mul_eax_overflow_to_edx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 100000 * 100000 = 10000000000 = 0x2540BE400
    // EDX = 0x2, EAX = 0x540BE400
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 100000;
    emu.regs_mut().rbx = 100000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x540BE400, "EAX (low dword)");
    assert_eq!(emu.regs().rdx, 0x00000002, "EDX (high dword)");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_mul_eax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "EAX = 0");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_eax_max_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0xFFFFFFFF * 0xFFFFFFFF = 0xFFFFFFFE00000001
    // EDX = 0xFFFFFFFE, EAX = 0x00000001
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000001, "EAX = 0x00000001");
    assert_eq!(emu.regs().rdx, 0xFFFFFFFE, "EDX = 0xFFFFFFFE");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_mul_eax_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rbx = 0x00000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x12345678, "EAX unchanged");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_eax_powers_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 65536 * 65536 = 4294967296 = 0x100000000
    // EDX = 0x1, EAX = 0x0
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 65536;
    emu.regs_mut().rbx = 65536;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000000, "EAX = 0");
    assert_eq!(emu.regs().rdx, 0x00000001, "EDX = 1");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_mul_ecx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xe1, 0xf4]; // MUL ECX
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rcx = 2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 2000000, "1000 * 2000 = 2000000");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
}

#[test]
fn test_mul_edx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xe2, 0xf4]; // MUL EDX
    emu.regs_mut().rax = 5;
    emu.regs_mut().rdx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 35, "5 * 7 = 35");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0 (result high word)");
}

// ============================================================================
// 64-bit MUL (opcode REX.W + F7 /4) - Result in RDX:RAX
// ============================================================================

#[test]
fn test_mul_rax_basic() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0xe3, // MUL RBX (REX.W F7 /4)
        0xf4,
    ];
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rbx = 2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 2000000, "RAX: 1000 * 2000 = 2000000");
    assert_eq!(emu.regs().rdx, 0, "RDX should be 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_mul_rax_overflow_to_rdx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x100000000 * 0x100000000 = 0x10000000000000000
    // RDX = 0x1, RAX = 0x0
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 0x100000000;
    emu.regs_mut().rbx = 0x100000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000000, "RAX (low qword) = 0");
    assert_eq!(emu.regs().rdx, 0x0000000000000001, "RDX (high qword) = 1");
    assert!(emu.flags().f_cf, "CF should be set");
    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_mul_rax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "RAX = 0");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_rax_max_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0xFFFFFFFFFFFFFFFF * 0xFFFFFFFFFFFFFFFF
    // = 0xFFFFFFFFFFFFFFFE0000000000000001
    // RDX = 0xFFFFFFFFFFFFFFFE, RAX = 0x0000000000000001
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rbx = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000001, "RAX (low)");
    assert_eq!(emu.regs().rdx, 0xFFFFFFFFFFFFFFFE, "RDX (high)");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_mul_rax_by_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rbx = 0x0000000000000001;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "RAX unchanged");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_rax_large_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0x1000000000000 * 0x1000000000000 = 0x1000000000000000000000000
    // 128-bit result, needs RDX:RAX
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 0x0001000000000000;
    emu.regs_mut().rbx = 0x0001000000000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000000, "RAX = 0");
    assert_eq!(emu.regs().rdx, 0x0000000100000000, "RDX contains high bits");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_mul_rcx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xe1, 0xf4]; // MUL RCX
    emu.regs_mut().rax = 123456789;
    emu.regs_mut().rcx = 987654321;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 123456789 * 987654321 = 121932631112635269
    assert_eq!(emu.regs().rax, 121932631112635269, "Product in RAX");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0 (fits in 64 bits)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_rdx_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xe2, 0xf4]; // MUL RDX
    emu.regs_mut().rax = 12345;
    emu.regs_mut().rdx = 67890;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 838102050, "12345 * 67890");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0 (result high word)");
}

// ============================================================================
// Extended registers (R8-R15) with REX prefix
// ============================================================================

#[test]
fn test_mul_r8b_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf6, 0xe0, // MUL R8B (REX.B F6 /4)
        0xf4,
    ];
    emu.regs_mut().rax = 20; // AL = 20
    emu.regs_mut().r8 = 10;  // R8B = 10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 200, "20 * 10 = 200 in AX");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_r9w_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0x41, 0xf7, 0xe1, // MUL R9W
        0xf4,
    ];
    emu.regs_mut().rax = 1000;
    emu.regs_mut().r9 = 50;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 50000, "AX: 1000 * 50 = 50000");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX = 0");
}

#[test]
fn test_mul_r10d_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x41, 0xf7, 0xe2, // MUL R10D
        0xf4,
    ];
    emu.regs_mut().rax = 50000;
    emu.regs_mut().r10 = 60000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 3000000000, "EAX: 50000 * 60000");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
}

#[test]
fn test_mul_r11_extended_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x49, 0xf7, 0xe3, // MUL R11 (REX.WB F7 /4)
        0xf4,
    ];
    emu.regs_mut().rax = 123456;
    emu.regs_mut().r11 = 789012;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 97408265472, "RAX: product");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
}

#[test]
fn test_mul_r15_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xf7, 0xe7, 0xf4]; // MUL R15
    emu.regs_mut().rax = 0x0000000100000000;
    emu.regs_mut().r15 = 0x0000000200000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should be set (overflow)");
    assert!(emu.flags().f_of, "OF should be set");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_mul_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0x25, 0xfa, 0x0f, 0x00, 0x00, // MUL BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 15);

    emu.regs_mut().rax = 20; // AL = 20

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 300, "20 * 15 = 300");
    assert!(emu.flags().f_cf, "CF should be set (overflow to AH)");
}

#[test]
fn test_mul_word_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0x25, 0xf9, 0x0f, 0x00, 0x00, // MUL WORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_word(DATA_ADDR, 100);

    emu.regs_mut().rax = 1000;

    emu.run(None).unwrap();

    // 1000 * 100 = 100000 = 0x186A0, AX = 0x86A0 = 34464, DX = 0x1
    assert_eq!(emu.regs().rax & 0xFFFF, 34464, "AX: low 16 bits of 1000 * 100");
    assert_eq!(emu.regs().rdx & 0xFFFF, 1, "DX: high 16 bits");
}

#[test]
fn test_mul_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0x25, 0xfa, 0x0f, 0x00, 0x00, // MUL DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 5000);

    emu.regs_mut().rax = 10000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 50000000, "EAX: 10000 * 5000");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
}

#[test]
fn test_mul_qword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0x25, 0xf9, 0x0f, 0x00, 0x00, // MUL QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 987654321);

    emu.regs_mut().rax = 123456789;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 121932631112635269, "RAX: product");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
}

// ============================================================================
// Flag behavior edge cases
// ============================================================================

#[test]
fn test_mul_flags_clear_when_no_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 10;
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 100, "10 * 10 = 100");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH = 0");
    assert!(!emu.flags().f_cf, "CF should be clear (AH is 0)");
    assert!(!emu.flags().f_of, "OF should be clear (AH is 0)");
}

#[test]
fn test_mul_flags_set_when_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 16;
    emu.regs_mut().rbx = 16;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 256, "16 * 16 = 256 = 0x0100");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 1, "AH = 1");
    assert!(emu.flags().f_cf, "CF should be set (AH is non-zero)");
    assert!(emu.flags().f_of, "OF should be set (AH is non-zero)");
}
