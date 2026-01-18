use crate::*;

// DIV — Unsigned Divide
//
// Opcodes:
// - F6 /6       DIV r/m8      AL := AX / r/m8; AH := AX % r/m8
// - F7 /6       DIV r/m16     AX := DX:AX / r/m16; DX := DX:AX % r/m16
// - F7 /6       DIV r/m32     EAX := EDX:EAX / r/m32; EDX := EDX:EAX % r/m32
// - REX.W+F7 /6 DIV r/m64     RAX := RDX:RAX / r/m64; RDX := RDX:RAX % r/m64
//
// Operation: dividend / divisor = quotient, remainder
//
// Flags: Undefined (not set by DIV)
//
// Exceptions:
// - #DE (Divide Error): if divisor is 0 or quotient doesn't fit
//
// CRITICAL: DIV works with UNSIGNED integers.
// For 8-bit:  AX (16-bit) / r/m8 (8-bit) -> AL (quotient), AH (remainder)
//             Max quotient: 255 (fits in AL)

// ============================================================================
// 8-bit DIV (opcode F6 /6)
// ============================================================================

#[test]
fn test_div_al_simple() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 100 / 10 = 10 remainder 0
    // AX = 100 (AH=0, AL=100)
    let code = [
        0xf6, 0xf3, // DIV BL (F6 /6, ModRM=11_110_011)
        0xf4,       // HLT
    ];
    emu.regs_mut().rax = 100;  // AX = 100 (AH=0, AL=100)
    emu.regs_mut().rbx = 10;   // BL = 10
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL (quotient) = 100 / 10 = 10");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH (remainder) = 100 % 10 = 0");
}

#[test]
fn test_div_al_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 100 / 7 = 14 remainder 2
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 100;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 14, "AL (quotient) = 100 / 7 = 14");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 2, "AH (remainder) = 100 % 7 = 2");
}

#[test]
fn test_div_al_max_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 255 / 1 = 255 remainder 0
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 255;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 255, "AL = 255 / 1 = 255");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH = 0");
}

#[test]
fn test_div_al_large_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 300 / 10 = 30 remainder 0
    // AX = 300 (0x012C)
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 300;
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 30, "AL (quotient) = 300 / 10 = 30");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH (remainder) = 0");
}

#[test]
fn test_div_al_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 5 / 1 = 5 remainder 0
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 5;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 5, "5 / 1 = 5");
}

// ============================================================================
// 16-bit DIV (opcode F7 /6 with 0x66 prefix)
// ============================================================================

#[test]
fn test_div_ax_simple() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000 / 10 = 100 remainder 0
    let code = [
        0x66, 0xf7, 0xf3, // DIV BX (66 F7 /6)
        0xf4,
    ];
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 100, "AX (quotient) = 1000 / 10 = 100");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX (remainder) = 0");
}

#[test]
fn test_div_ax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000 / 7 = 142 remainder 6
    let code = [0x66, 0xf7, 0xf3, 0xf4]; // DIV BX
    emu.regs_mut().rax = 1000;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 142, "AX (quotient)");
    assert_eq!(emu.regs().rdx & 0xFFFF, 6, "DX (remainder)");
}

#[test]
fn test_div_ax_dx_nonzero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // (DX:AX) = 0x00011000 (69632) / 100 = 696 remainder 32
    let code = [0x66, 0xf7, 0xf3, 0xf4]; // DIV BX
    emu.regs_mut().rax = 0x1000;  // 4096
    emu.regs_mut().rdx = 0x0001;  // High 16 bits = 1
    emu.regs_mut().rbx = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 0x00011000 = 69632
    // 69632 / 100 = 696 remainder 32
    assert_eq!(emu.regs().rax & 0xFFFF, 696, "AX (quotient)");
    assert_eq!(emu.regs().rdx & 0xFFFF, 32, "DX (remainder)");
}

#[test]
fn test_div_ax_max_quotient() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 65535 / 1 = 65535 remainder 0 (max 16-bit)
    let code = [0x66, 0xf7, 0xf3, 0xf4]; // DIV BX
    emu.regs_mut().rax = 0xFFFF;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFF, 0xFFFF, "AX = 65535");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX = 0");
}

// ============================================================================
// 32-bit DIV (opcode F7 /6)
// ============================================================================

#[test]
fn test_div_eax_simple() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000 / 1000 = 1000 remainder 0
    let code = [
        0xf7, 0xf3, // DIV EBX (F7 /6)
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
fn test_div_eax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000 / 7 = 142857 remainder 1
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 142857, "EAX (quotient)");
    assert_eq!(emu.regs().rdx, 1, "EDX (remainder)");
}

#[test]
fn test_div_eax_edx_nonzero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // (EDX:EAX) / divisor
    // EDX=0x00000001, EAX=0x00000000 = 0x100000000 = 4294967296
    // 4294967296 / 100 = 42949672 remainder 96
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rdx = 0x00000001;
    emu.regs_mut().rbx = 100;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 42949672, "EAX (quotient)");
    assert_eq!(emu.regs().rdx, 96, "EDX (remainder)");
}

#[test]
fn test_div_eax_max_quotient() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0xFFFFFFFF / 1 = 0xFFFFFFFF remainder 0
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 0xFFFFFFFF;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "EAX = max 32-bit");
    assert_eq!(emu.regs().rdx, 0, "EDX = 0");
}

#[test]
fn test_div_eax_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1234567 / 1 = 1234567 remainder 0
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 1234567;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1234567, "quotient");
    assert_eq!(emu.regs().rdx, 0, "remainder");
}

// ============================================================================
// 64-bit DIV (opcode REX.W + F7 /6)
// ============================================================================

#[test]
fn test_div_rax_simple() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000000000 / 1000000 = 1000000 remainder 0
    let code = [
        0x48, 0xf7, 0xf3, // DIV RBX (REX.W F7 /6)
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
fn test_div_rax_with_remainder() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1000000000000 / 7 = 142857142857 remainder 1
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    emu.regs_mut().rax = 1000000000000;
    emu.regs_mut().rbx = 7;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 142857142857, "RAX (quotient)");
    assert_eq!(emu.regs().rdx, 1, "RDX (remainder)");
}

#[test]
fn test_div_rax_rdx_nonzero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // (RDX:RAX) = 0x0000000000000001:0x0000000000000000 = 2^64
    // 2^64 / 0x100000001 = 0xFFFFFFFF remainder 1
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    emu.regs_mut().rax = 0x0000000000000000;
    emu.regs_mut().rdx = 0x0000000000000001;  // RDX:RAX = 2^64
    emu.regs_mut().rbx = 0x100000001;  // Divisor = 2^32 + 1 = 4294967297
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 2^64 / (2^32 + 1) = 4294967295 remainder 1
    assert_eq!(emu.regs().rax, 0xFFFFFFFF, "RAX (quotient)");
    assert_eq!(emu.regs().rdx, 1, "RDX (remainder)");
}

#[test]
fn test_div_rax_max_quotient() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 0xFFFFFFFFFFFFFFFF / 1 = max remainder 0
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    emu.regs_mut().rax = 0xFFFFFFFFFFFFFFFF;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFFFFFFFFFFFFFF, "RAX = max 64-bit");
    assert_eq!(emu.regs().rdx, 0, "RDX = 0");
}

#[test]
fn test_div_rax_one() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 123456789123456789 / 1
    let code = [0x48, 0xf7, 0xf3, 0xf4]; // DIV RBX
    emu.regs_mut().rax = 123456789123456789;
    emu.regs_mut().rbx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 123456789123456789, "quotient");
    assert_eq!(emu.regs().rdx, 0, "remainder");
}

// ============================================================================
// Different registers
// ============================================================================

#[test]
fn test_div_cl_register() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // DIV CL (8-bit) - ModRM 11_110_001 = 0xF1
    let code = [0xf6, 0xf1, 0xf4];
    emu.regs_mut().rax = 100;
    emu.regs_mut().rcx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL (quotient)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH (remainder)");
}

#[test]
fn test_div_cx_16bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // DIV CX (16-bit) - ModRM 11_110_001 = 0xF1
    let code = [0x66, 0xf7, 0xf1, 0xf4]; // DIV CX
    emu.regs_mut().rax = 10000;  // AX = 10000
    emu.regs_mut().rdx = 0;      // DX = 0 (high part of dividend)
    emu.regs_mut().rcx = 100;    // CX = 100 (divisor)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // DX:AX = 0:10000 = 10000, divided by 100 = quotient 100, remainder 0
    assert_eq!(emu.regs().rax & 0xFFFF, 100, "AX (quotient)");
    assert_eq!(emu.regs().rdx & 0xFFFF, 0, "DX (remainder)");
}

#[test]
fn test_div_ecx_32bit() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // DIV ECX (32-bit) - ModRM 11_110_001 = 0xF1
    let code = [0xf7, 0xf1, 0xf4]; // DIV ECX
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rdx = 0;
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
fn test_div_r8b() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xf6, 0xf0, 0xf4]; // DIV R8B (ModRM 11_110_000 = 0xF0)
    emu.regs_mut().rax = 100;
    emu.regs_mut().r8 = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL (quotient)");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH (remainder)");
}

#[test]
fn test_div_r10d() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x41, 0xf7, 0xf2, 0xf4]; // DIV R10D (ModRM 11_110_010 = 0xF2)
    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r10 = 1000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000, "EAX (quotient)");
    assert_eq!(emu.regs().rdx, 0, "EDX (remainder)");
}

#[test]
fn test_div_r15() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [0x49, 0xf7, 0xf7, 0xf4]; // DIV R15 (ModRM 11_110_111 = 0xF7)
    emu.regs_mut().rax = 1000000000000;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().r15 = 1000000;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000000, "RAX (quotient)");
    assert_eq!(emu.regs().rdx, 0, "RDX (remainder)");
}

// ============================================================================
// Memory operands
// ============================================================================

#[test]
fn test_div_byte_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf6, 0x35, 0xfa, 0x0f, 0x00, 0x00, // DIV BYTE PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_byte(DATA_ADDR, 10);

    emu.regs_mut().rax = 100;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 10, "AL (quotient) = 100 / 10");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 0, "AH (remainder)");
}

#[test]
fn test_div_dword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xf7, 0x35, 0xfa, 0x0f, 0x00, 0x00, // DIV DWORD PTR [rip+0x0FFA]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_dword(DATA_ADDR, 1000);

    emu.regs_mut().rax = 1000000;
    emu.regs_mut().rdx = 0;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000, "EAX (quotient) = 1000000 / 1000");
    assert_eq!(emu.regs().rdx, 0, "EDX (remainder)");
}

#[test]
fn test_div_qword_ptr_mem() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xf7, 0x35, 0xf9, 0x0f, 0x00, 0x00, // DIV QWORD PTR [rip+0x0FF9]
        0xf4,
    ];
    emu.load_code_bytes(&code);
    emu.maps.write_qword(DATA_ADDR, 1000000);

    emu.regs_mut().rax = 1000000000000;
    emu.regs_mut().rdx = 0;

    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 1000000, "RAX (quotient)");
    assert_eq!(emu.regs().rdx, 0, "RDX (remainder)");
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_div_small_dividend() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 5 / 10 = 0 remainder 5
    let code = [0xf6, 0xf3, 0xf4]; // DIV BL
    emu.regs_mut().rax = 5;
    emu.regs_mut().rbx = 10;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0, "AL (quotient) = 0");
    assert_eq!((emu.regs().rax >> 8) & 0xFF, 5, "AH (remainder) = 5");
}

#[test]
fn test_div_power_of_two() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // 1024 / 256 = 4 remainder 0
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 1024;
    emu.regs_mut().rdx = 0;
    emu.regs_mut().rbx = 256;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 4, "quotient");
    assert_eq!(emu.regs().rdx, 0, "remainder");
}

#[test]
fn test_div_result_in_upper() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // (EDX:EAX) = (0x00000002, 0x00000000) = 0x200000000 = 2^33
    // 2^33 / 0x80000000 = 4 remainder 0
    let code = [0xf7, 0xf3, 0xf4]; // DIV EBX
    emu.regs_mut().rax = 0x00000000;
    emu.regs_mut().rdx = 0x00000002;  // EDX:EAX = 2 * 2^32 = 2^33
    emu.regs_mut().rbx = 0x80000000;  // Divisor = 2^31
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    // 2^33 / 2^31 = 4 remainder 0
    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 4, "EAX (quotient)");
    assert_eq!(emu.regs().rdx & 0xFFFFFFFF, 0, "EDX (remainder)");
}
