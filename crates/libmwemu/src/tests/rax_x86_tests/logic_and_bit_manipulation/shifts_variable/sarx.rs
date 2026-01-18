use crate::*;

// SARX - Arithmetic Right Shift Without Affecting Flags (BMI2)
// This file contains comprehensive tests for the SARX instruction.
//
// SARX performs an arithmetic right shift operation without modifying any flags.
// It is part of the BMI2 (Bit Manipulation Instruction Set 2) extension.
//
// Syntax: SARX dest, src, count
// - dest: destination register (receives shifted result)
// - src: source operand (register or memory) to be shifted
// - count: register containing shift count (low 5/6 bits used)
//
// Opcodes:
// VEX.LZ.F3.0F38.W0 F7 /r   SARX r32, r/m32, r32   - 32-bit arithmetic right shift
// VEX.LZ.F3.0F38.W1 F7 /r   SARX r64, r/m64, r64   - 64-bit arithmetic right shift
//
// Count Masking:
// - 32-bit: count masked to 5 bits (0-31)
// - 64-bit: count masked to 6 bits (0-63)
//
// Unlike SHRX, SARX sign-extends. The sign bit is replicated from the left.
//
// Flags: None modified (unlike SAR which sets CF, ZF, SF, OF)

// ============================================================================
// SARX 32-bit - Comprehensive Shift Count Tests
// ============================================================================

#[test]
fn test_sarx_32bit_shift_by_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "Shift by 0 should preserve value");
}

#[test]
fn test_sarx_32bit_shift_positive_by_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000100;
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000080, "256 >> 1 = 128 (positive)");
}

#[test]
fn test_sarx_32bit_shift_negative_by_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // -2147483648 in signed 32-bit
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0xC0000000,
        "Sign bit should be extended"
    );
}

#[test]
fn test_sarx_32bit_all_shift_counts_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for count in 0..=31 {
        let code = [
            0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = 0x7FFFFFFF; // Max positive 32-bit
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = (0x7FFFFFFFi32 >> count) as u32;
        assert_eq!(
            emu.regs().rax & 0xFFFFFFFF,
            expected as u64,
            "Positive value >> {} failed",
            count
        );
    }
}

#[test]
fn test_sarx_32bit_all_shift_counts_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for count in 0..=31 {
        let code = [
            0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = 0x80000000; // Min negative 32-bit
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = (0x80000000u32 as i32 >> count) as u32;
        assert_eq!(
            emu.regs().rax & 0xFFFFFFFF,
            expected as u64,
            "Negative value >> {} failed",
            count
        );
    }
}

#[test]
fn test_sarx_32bit_negative_shift_to_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000;
    emu.regs_mut().rcx = 31;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0xFFFFFFFF,
        "Negative shifted 31 times becomes all ones"
    );
}

#[test]
fn test_sarx_32bit_count_masking() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 32; // Masked to 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0x12345678,
        "Count 32 masked to 0, value unchanged"
    );
}

#[test]
fn test_sarx_32bit_sign_extension_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0xF0000000u32, 4, 0xFF000000u32),
        (0xF0000000u32, 8, 0xFFF00000u32),
        (0xF0000000u32, 12, 0xFFFF0000u32),
        (0xF0000000u32, 16, 0xFFFFF000u32),
        (0xF0000000u32, 20, 0xFFFFFF00u32),
        (0xF0000000u32, 24, 0xFFFFFFF0u32),
        (0xF0000000u32, 28, 0xFFFFFFFFu32),
    ];

    for (value, count, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = value as u64;
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(
            emu.regs().rax & 0xFFFFFFFF,
            expected as u64,
            "Sign extension for 0x{:08X} >> {} failed",
            value,
            count
        );
    }
}

// ============================================================================
// SARX 64-bit - Comprehensive Shift Count Tests
// ============================================================================

#[test]
fn test_sarx_64bit_shift_by_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "Shift by 0 should preserve value");
}

#[test]
fn test_sarx_64bit_shift_positive_by_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000000000100;
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000080, "256 >> 1 = 128 (positive)");
}

#[test]
fn test_sarx_64bit_shift_negative_by_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000000000000000; // Min negative 64-bit
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xC000000000000000, "Sign bit should be extended");
}

#[test]
fn test_sarx_64bit_all_shift_counts_positive() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for count in 0..=63 {
        let code = [
            0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
            0xf4,
        ];
        emu.regs_mut().rbx = 0x7FFFFFFFFFFFFFFF; // Max positive 64-bit
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = (0x7FFFFFFFFFFFFFFFi64 >> count) as u64;
        assert_eq!(emu.regs().rax, expected, "Positive value >> {} failed", count);
    }
}

#[test]
fn test_sarx_64bit_all_shift_counts_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for count in 0..=63 {
        let code = [
            0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
            0xf4,
        ];
        emu.regs_mut().rbx = 0x8000000000000000; // Min negative 64-bit
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = (0x8000000000000000u64 as i64 >> count) as u64;
        assert_eq!(emu.regs().rax, expected, "Negative value >> {} failed", count);
    }
}

#[test]
fn test_sarx_64bit_negative_shift_to_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000000000000000;
    emu.regs_mut().rcx = 63;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax,
        0xFFFFFFFFFFFFFFFF,
        "Negative shifted 63 times becomes all ones"
    );
}

#[test]
fn test_sarx_64bit_count_masking() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 64; // Masked to 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "Count 64 masked to 0");
}

#[test]
fn test_sarx_64bit_sign_extension_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0xF000000000000000u64, 4, 0xFF00000000000000u64),
        (0xF000000000000000u64, 8, 0xFFF0000000000000u64),
        (0xF000000000000000u64, 16, 0xFFFFF00000000000u64),
        (0xF000000000000000u64, 32, 0xFFFFFFFFF0000000u64),
        (0xF000000000000000u64, 48, 0xFFFFFFFFFFFFF000u64),
        (0xF000000000000000u64, 56, 0xFFFFFFFFFFFFFFF0u64),
        (0xF000000000000000u64, 60, 0xFFFFFFFFFFFFFFFFu64),
    ];

    for (value, count, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
            0xf4,
        ];
        emu.regs_mut().rbx = value;
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(
            emu.regs().rax, expected,
            "Sign extension for 0x{:016X} >> {} failed",
            value, count
        );
    }
}

// ============================================================================
// Flag Tests - SARX Does NOT Modify Flags
// ============================================================================

#[test]
fn test_sarx_32bit_does_not_modify_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SARX should not modify CF
    let code = [
        0xf9,                         // STC (set CF)
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000;
    emu.regs_mut().rcx = 4;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should still be set from STC");
}

#[test]
fn test_sarx_64bit_preserves_all_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02,                   // SUB RAX, 2 (sets CF, SF, AF)
        0xc4, 0xe2, 0xf2, 0xf7, 0xc3,             // SARX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000000000000000;
    emu.regs_mut().rcx = 16;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFFFF800000000000, "Shift result correct");
    assert!(emu.flags().f_cf, "CF should still be set from SUB");
    assert!(emu.flags().f_sf, "SF should still be set from SUB");
}

// ============================================================================
// Pattern Tests
// ============================================================================

#[test]
fn test_sarx_32bit_all_ones_stays_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for count in vec![1, 4, 8, 16, 24, 31] {
        let code = [
            0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = 0xFFFFFFFF;
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(
            emu.regs().rax & 0xFFFFFFFF,
            0xFFFFFFFF,
            "All ones should remain all ones after SARX by {}",
            count
        );
    }
}

#[test]
fn test_sarx_64bit_all_ones_stays_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for count in vec![1, 8, 16, 32, 48, 63] {
        let code = [
            0xc4, 0xe2, 0xf2, 0xf7, 0xc3, // SARX RAX, RBX, RCX
            0xf4,
        ];
        emu.regs_mut().rbx = 0xFFFFFFFFFFFFFFFF;
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(
            emu.regs().rax,
            0xFFFFFFFFFFFFFFFF,
            "All ones should remain all ones after SARX by {}",
            count
        );
    }
}

#[test]
fn test_sarx_32bit_positive_no_sign_extension() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x7FFFFFFF; // Max positive
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0x3FFFFFFF,
        "Positive should not sign extend"
    );
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_sarx_32bit_memory_operand_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SARX with memory source (negative)
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SARX EAX, [0x2000], ECX
        0xf4,
    ];
    emu.regs_mut().rcx = 4;
    emu.load_code_bytes(&code);
    use crate::*;
    emu.maps.write_dword(DATA_ADDR, 0x80000000);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0xF8000000,
        "Memory operand with sign extension"
    );
}

#[test]
fn test_sarx_64bit_memory_operand_negative() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SARX 64-bit with memory source (negative)
    let code = [
        0xc4, 0xe2, 0xf2, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SARX RAX, [0x2000], RCX
        0xf4,
    ];
    emu.regs_mut().rcx = 8;
    emu.load_code_bytes(&code);
    use crate::*;
    emu.maps.write_qword(DATA_ADDR, 0x8000000000000000);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0xFF80000000000000, "64-bit memory operand sign extended");
}

// ============================================================================
// Extended Register Tests (R8-R15)
// ============================================================================

#[test]
fn test_sarx_32bit_r8d_r9d_r10d() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SARX R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x2a, 0xf7, 0xc1, // SARX R8D, R9D, R10D
        0xf4,
    ];
    emu.regs_mut().r9 = 0x80000000;
    emu.regs_mut().r10 = 4;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().r8 & 0xFFFFFFFF,
        0xF8000000,
        "Extended registers work correctly"
    );
}

#[test]
fn test_sarx_64bit_r14_r15_r13() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SARX R14, R15, R13
    let code = [
        0xc4, 0x42, 0x92, 0xf7, 0xf7, // SARX R14, R15, R13
        0xf4,
    ];
    emu.regs_mut().r15 = 0x8000000000000000;
    emu.regs_mut().r13 = 8;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r14, 0xFF80000000000000, "64-bit extended registers");
}

// ============================================================================
// Edge Cases and Comparisons
// ============================================================================

#[test]
fn test_sarx_source_preservation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000;
    emu.regs_mut().rcx = 8;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x80000000, "Source unchanged");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 8, "Count unchanged");
}

#[test]
fn test_sarx_consecutive_shifts() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0x48, 0x89, 0xc3,             // MOV RBX, RAX
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000;
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0xE0000000,
        "Consecutive SARX operations"
    );
}

#[test]
fn test_sarx_vs_shrx_difference() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x72, 0xf7, 0xc3, // SARX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xF0000000; // Negative
    emu.regs_mut().rcx = 4;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0xFF000000,
        "SARX sign-extends (unlike SHRX which would give 0x0F000000)"
    );
}
