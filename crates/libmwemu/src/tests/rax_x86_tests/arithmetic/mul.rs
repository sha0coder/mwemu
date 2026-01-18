use crate::*;

// MUL — Unsigned Multiply
//
// Opcodes:
// - F6 /4       MUL r/m8      AX := AL * r/m8
// - F7 /4       MUL r/m16     DX:AX := AX * r/m16
// - F7 /4       MUL r/m32     EDX:EAX := EAX * r/m32
// - REX.W+F7 /4 MUL r/m64     RDX:RAX := RAX * r/m64
//
// Operation: For 8-bit:  AX := AL * r/m8
//            For 16-bit: DX:AX := AX * r/m16
//            For 32-bit: EDX:EAX := EAX * r/m32
//            For 64-bit: RDX:RAX := RAX * r/m64
//
// Flags: CF and OF are set if the result is nonzero in the upper half of the destination
//        SF, ZF, AF, PF are undefined
//
// CRITICAL: MUL works with UNSIGNED integers, unlike IMUL which is signed

// ============================================================================
// 8-bit MUL (opcode F6 /4)
// ============================================================================

#[test]
fn test_mul_al_small() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0xe3, // MUL BL (F6 /4, ModRM=11_100_011)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 5;  // AL = 5
    emu.regs_mut().rbx = 3;  // BL = 3
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 15, "5 * 3 = 15");
    assert!(!emu.flags().f_cf, "CF should be clear (fits in AL)");
    assert!(!emu.flags().f_of, "OF should be clear");
}

#[test]
fn test_mul_al_max() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 255 * 255 = 65025 (0xFE01) - upper byte not zero
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 255;
    emu.regs_mut().rbx = 255;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFE01, "255 * 255 = 65025 (0xFE01)");
    assert!(emu.flags().f_cf, "CF should be set (result in AH)");
    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_mul_al_fits_in_byte() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 10 * 15 = 150 (0x96), fits in AX with AH=0
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 10;
    emu.regs_mut().rbx = 15;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 150, "10 * 15 = 150");
    assert!(!emu.flags().f_cf, "CF should be clear (result fits in AL)");
}

#[test]
fn test_mul_al_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "0 * 100 = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_al_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf6, 0xe3, 0xf4]; // MUL BL
    emu.regs_mut().rax = 1;
    emu.regs_mut().rbx = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 100, "1 * 100 = 100");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

// ============================================================================
// 16-bit MUL (opcode F7 /4 with 0x66 prefix)
// ============================================================================

#[test]
fn test_mul_ax_small() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x66, 0xf7, 0xe3, // MUL BX (66 F7 /4)
        0xf4,
    ];
    emu.regs_mut().rax = 100;
    emu.regs_mut().rbx = 50;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 5000, "AX: 100 * 50 = 5000");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX = 0 (upper is zero)");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_ax_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 65535 * 65535 = 0xFFFE0001
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    emu.regs_mut().rax = 0xFFFF;
    emu.regs_mut().rbx = 0xFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0x0001, "AX (low word)");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0xFFFE, "DX (high word, non-zero)");
    assert!(emu.flags().f_cf, "CF should be set (overflow)");
    assert!(emu.flags().f_of, "OF should be set");
}

#[test]
fn test_mul_ax_fits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000 * 50 = 50000 (fits in 16 bits)
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rbx = 50;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 50000, "1000 * 50 = 50000");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_ax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x66, 0xf7, 0xe3, 0xf4]; // MUL BX
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0, "0 * 100 = 0");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

// ============================================================================
// 32-bit MUL (opcode F7 /4)
// ============================================================================

#[test]
fn test_mul_eax_small() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0xe3, // MUL EBX (F7 /4)
        0xf4,
    ];
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rbx = 2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 2000000, "EAX: 1000 * 2000 = 2000000");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_eax_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0xFFFFFFFF * 0xFFFFFFFF
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000001, "EAX (low)");
    assert_eq!(emu.regs().rdx, 0xFFFFFFFE, "EDX (high)");
    assert!(emu.flags().f_cf, "CF should be set (overflow)");
}

#[test]
fn test_mul_eax_fits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 100000 * 50000 = 5000000000 (fits in 32 bits? No, exceeds)
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 100000;
    emu.regs_mut().rbx = 50000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 5000000000 = 0x12A05F200, so EDX has upper bits
    assert!(emu.flags().f_cf, "CF should be set (result > 32-bit)");
}

#[test]
fn test_mul_eax_small_product() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 100 * 200 = 20000 (fits in 32 bits)
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 100;
    emu.regs_mut().rbx = 200;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 20000, "100 * 200 = 20000");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_eax_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 0;
    emu.regs_mut().rbx = 1000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0, "0 * 1000000 = 0");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

// ============================================================================
// 64-bit MUL (opcode REX.W + F7 /4)
// ============================================================================

#[test]
fn test_mul_rax_small() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0xe3, // MUL RBX (REX.W F7 /4)
        0xf4,
    ];
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rbx = 2000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 2000000000000, "RAX: 1M * 2M");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_rax_overflow() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0xFFFFFFFFFFFFFFFF * 0xFFFFFFFFFFFFFFFF
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rbx = 0xFFFFFFFFFFFFFFFF;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000001, "RAX (low)");
    assert_eq!(emu.regs().rdx, 0xFFFFFFFFFFFFFFFE, "RDX (high)");
    assert!(emu.flags().f_cf, "CF should be set (overflow)");
}

#[test]
fn test_mul_rax_large_product() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 2^32 * 2^32 = 2^64 = 0x1_00000000_00000000 as 128-bit
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 0x0000000100000000; // 2^32
    emu.regs_mut().rbx = 0x0000000100000000; // 2^32
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000000, "RAX (low 64 bits = 0)");
    assert_eq!(emu.regs().rdx, 0x0000000000000001, "RDX (high 64 bits = 1)");
    assert!(emu.flags().f_cf, "CF should be set");
}

#[test]
fn test_mul_rax_fits() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000 * 2000 = 2000000 (fits in 64 bits)
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rbx = 2000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 2000000, "1000 * 2000 = 2000000");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
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

    assert_eq!(emu.regs().rax, 0, "0 * max = 0");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_rax_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x48, 0xf7, 0xe3, 0xf4]; // MUL RBX
    emu.regs_mut().rax = 1;
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "1 * x = x");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

// ============================================================================
// Different registers
// ============================================================================

#[test]
fn test_mul_cl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // MUL CL (8-bit)
    let code = [0xf6, 0xe1, 0xf4];
    emu.regs_mut().rax = 20;
    emu.regs_mut().rcx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 200, "20 * 10 = 200");
}

#[test]
fn test_mul_dx_16bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // MUL DX (16-bit): DX:AX = AX * DX
    // 1000 * 100 = 100000 = 0x000186A0
    // DX = 0x0001, AX = 0x86A0
    let code = [0x66, 0xf7, 0xe2, 0xf4]; // MUL DX
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rdx = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    let result = ((emu.regs().rdx & 0xFFFF) << 16) | (emu.regs().rax & 0xFFFF);
    assert_eq!(result, 100000, "1000 * 100 = 100000 (in DX:AX)");
}

#[test]
fn test_mul_ecx_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // MUL ECX (32-bit)
    let code = [0xf7, 0xe1, 0xf4]; // MUL ECX
    emu.regs_mut().rax = 100000;
    emu.regs_mut().rcx = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 10000000, "100000 * 100 = 10000000");
}

// ============================================================================
// Extended registers (R8-R15)
// ============================================================================

#[test]
fn test_mul_r8b() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xf6, 0xe0, 0xf4]; // MUL R8B
    emu.regs_mut().rax = 25;
    emu.regs_mut().r8 = 4;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 100, "25 * 4 = 100");
}

#[test]
fn test_mul_r10d() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xf7, 0xe2, 0xf4]; // MUL R10D
    emu.regs_mut().rax = 1000;
    emu.regs_mut().r10 = 5000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 5000000, "1000 * 5000 = 5000000");
}

#[test]
fn test_mul_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xf7, 0xe7, 0xf4]; // MUL R15
    emu.regs_mut().rax = 100;
    emu.regs_mut().r15 = 200;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 20000, "100 * 200 = 20000");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
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
    emu.maps.write_byte(DATA_ADDR, 25);

    emu.regs_mut().rax = 4;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 100, "4 * 25 = 100");
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
    emu.maps.write_word(DATA_ADDR, 1000);

    emu.regs_mut().rax = 50;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 50000, "50 * 1000 = 50000");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX = 0");
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
    emu.maps.write_dword(DATA_ADDR, 100000);

    emu.regs_mut().rax = 100;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 10000000, "100 * 100000 = 10000000");
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
    emu.maps.write_qword(DATA_ADDR, 1000000);

    emu.regs_mut().rax = 2000000;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 2000000000000, "2000000 * 1000000");
}

// ============================================================================
// Comparison: MUL vs IMUL with unsigned values
// ============================================================================

#[test]
fn test_mul_vs_imul_unsigned() {
    let DATA_ADDR = 0x7000;
    // but CF/OF flags may differ

    // MUL: 100 * 200 = 20000
    let code_mul = [0xf7, 0xe3, 0xf4]; // MUL EBX
    let mut emu1 = emu64();
    emu1.regs_mut().rax = 100;
    emu1.regs_mut().rbx = 200;
    emu1.load_code_bytes(&code_mul);
    emu1.run(None).unwrap();

    // IMUL (two-operand): EBX = EBX * EAX
    let code_imul = [0x0f, 0xaf, 0xd8, 0xf4];
    let mut emu2 = emu64();
    emu2.regs_mut().rax = 100;
    emu2.regs_mut().rbx = 200;
    emu2.load_code_bytes(&code_imul);
    emu2.run(None).unwrap();

    assert_eq!(emu1.regs().rax, emu2.regs().rbx, "Products should match");
    assert_eq!(emu1.regs().rax, 20000, "Product is 20000");
}


// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_mul_powers_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // MUL EAX by powers of 2
    // 1000 * 256 = 256000
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rbx = 256;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 256000, "1000 * 256 = 256000");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}

#[test]
fn test_mul_boundary_values() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 65536 * 65535 = 4294836224 (just under 2^32)
    let code = [0xf7, 0xe3, 0xf4]; // MUL EBX
    emu.regs_mut().rax = 65536;
    emu.regs_mut().rbx = 65535;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 65536 * 65535 = 0xFFFF0000
    assert_eq!(emu.regs().rax, 0xFFFF0000, "Result");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
    assert!(!emu.flags().f_cf, "CF should be clear");
}
