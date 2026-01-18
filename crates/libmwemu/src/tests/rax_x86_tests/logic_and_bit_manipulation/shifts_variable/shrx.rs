use crate::*;

// SHRX - Logical Right Shift Without Affecting Flags (BMI2)
// This file contains comprehensive tests for the SHRX instruction.
//
// SHRX performs a logical right shift operation without modifying any flags.
// It is part of the BMI2 (Bit Manipulation Instruction Set 2) extension.
//
// Syntax: SHRX dest, src, count
// - dest: destination register (receives shifted result)
// - src: source operand (register or memory) to be shifted
// - count: register containing shift count (low 5/6 bits used)
//
// Opcodes:
// VEX.LZ.F2.0F38.W0 F7 /r   SHRX r32, r/m32, r32   - 32-bit logical right shift
// VEX.LZ.F2.0F38.W1 F7 /r   SHRX r64, r/m64, r64   - 64-bit logical right shift
//
// Count Masking:
// - 32-bit: count masked to 5 bits (0-31)
// - 64-bit: count masked to 6 bits (0-63)
//
// Unlike SARX, SHRX does NOT sign-extend. Zero bits are shifted in from the left.
//
// Flags: None modified (unlike SHR which sets CF, ZF, SF, OF)

// ============================================================================
// SHRX 32-bit - Comprehensive Shift Count Tests
// ============================================================================

#[test]
fn test_shrx_32bit_shift_by_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x12345678, "Shift by 0 should preserve value");
}

#[test]
fn test_shrx_32bit_shift_by_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000100;
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000080, "256 >> 1 = 128");
}

#[test]
fn test_shrx_32bit_all_shift_counts() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for count in 0..=31 {
        let code = [
            0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = 0x80000000;
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = 0x80000000u32 >> count;
        assert_eq!(
            emu.regs().rax & 0xFFFFFFFF,
            expected as u64,
            "0x80000000 >> {} should be 0x{:08X}",
            count,
            expected
        );
    }
}

#[test]
fn test_shrx_32bit_no_sign_extension() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHRX should NOT sign-extend (unlike SARX)
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000; // Sign bit set
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0x40000000,
        "Should NOT sign-extend, just shift in zero"
    );
}

#[test]
fn test_shrx_32bit_shift_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000;
    emu.regs_mut().rcx = 31;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000001, "High bit shifted to bit 0");
}

#[test]
fn test_shrx_32bit_count_masking() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
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
fn test_shrx_32bit_count_masking_33() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000A;
    emu.regs_mut().rcx = 33; // Masked to 1
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00000005, "10 >> 1 = 5");
}

#[test]
fn test_shrx_32bit_power_of_two_shifts() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let test_cases = vec![
        (0x80000000, 0, 0x80000000),
        (0x80000000, 1, 0x40000000),
        (0x80000000, 2, 0x20000000),
        (0x80000000, 4, 0x08000000),
        (0x80000000, 8, 0x00800000),
        (0x80000000, 16, 0x00008000),
        (0x80000000, 24, 0x00000080),
    ];

    for (value, count, expected) in test_cases {
        let code = [
            0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
            0xf4,
        ];
        emu.regs_mut().rbx = value;
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        assert_eq!(
            emu.regs().rax & 0xFFFFFFFF,
            expected,
            "0x{:08X} >> {} should be 0x{:08X}",
            value,
            count,
            expected
        );
    }
}

// ============================================================================
// SHRX 64-bit - Comprehensive Shift Count Tests
// ============================================================================

#[test]
fn test_shrx_64bit_shift_by_0() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 0;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "Shift by 0 should preserve value");
}

#[test]
fn test_shrx_64bit_shift_by_1() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x0000000000000100;
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000080, "256 >> 1 = 128");
}

#[test]
fn test_shrx_64bit_all_shift_counts() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    for count in 0..=63 {
        let code = [
            0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
            0xf4,
        ];
        emu.regs_mut().rbx = 0x8000000000000000;
        emu.regs_mut().rcx = count;
        emu.load_code_bytes(&code);
    emu.run(None).unwrap();

        let expected = 0x8000000000000000u64 >> count;
        assert_eq!(
            emu.regs().rax, expected,
            "0x8000000000000000 >> {} should be 0x{:016X}",
            count, expected
        );
    }
}

#[test]
fn test_shrx_64bit_no_sign_extension() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHRX should NOT sign-extend (unlike SARX)
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000000000000000; // Sign bit set
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x4000000000000000, "Should NOT sign-extend");
}

#[test]
fn test_shrx_64bit_shift_to_zero() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000000000000000;
    emu.regs_mut().rcx = 63;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000000000000001, "High bit shifted to bit 0");
}

#[test]
fn test_shrx_64bit_count_masking() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 64; // Masked to 0
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x123456789ABCDEF0, "Count 64 masked to 0");
}

#[test]
fn test_shrx_64bit_shift_by_32() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF00000000;
    emu.regs_mut().rcx = 32;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x00000000FFFFFFFF, "Upper dword shifted to lower");
}

// ============================================================================
// Flag Tests - SHRX Does NOT Modify Flags
// ============================================================================

#[test]
fn test_shrx_32bit_does_not_modify_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHRX should not modify CF even when bits shift out
    let code = [
        0xf9,                         // STC (set CF)
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x00000001; // Low bit set, will shift out
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "Low bit shifted out");
    assert!(emu.flags().f_cf, "CF should still be set from STC");
}

#[test]
fn test_shrx_32bit_does_not_clear_cf() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHRX should not clear CF
    let code = [
        0xf9,                         // STC (set CF)
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x80000000;
    emu.regs_mut().rcx = 8;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert!(emu.flags().f_cf, "CF should remain set");
}

#[test]
fn test_shrx_64bit_preserves_all_flags() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00, // MOV RAX, 1
        0x48, 0x83, 0xe8, 0x02,                   // SUB RAX, 2 (sets CF, SF, AF)
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3,             // SHRX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x8000000000000000;
    emu.regs_mut().rcx = 16;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0000800000000000, "Shift result correct");
    assert!(emu.flags().f_cf, "CF should still be set from SUB");
    assert!(emu.flags().f_sf, "SF should still be set from SUB");
}

// ============================================================================
// Pattern Tests
// ============================================================================

#[test]
fn test_shrx_32bit_alternating_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAA; // 10101010...
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0x55555555,
        "Alternating pattern shifted right by 1"
    );
}

#[test]
fn test_shrx_32bit_all_ones() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xFFFFFFFF;
    emu.regs_mut().rcx = 8;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00FFFFFF, "All ones shifted right by 8");
}

#[test]
fn test_shrx_64bit_alternating_pattern() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0xc3, // SHRX RAX, RBX, RCX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xAAAAAAAAAAAAAAAA;
    emu.regs_mut().rcx = 1;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x5555555555555555, "Alternating pattern shifted");
}

// ============================================================================
// Memory Operand Tests
// ============================================================================

#[test]
fn test_shrx_32bit_memory_operand() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHRX with memory source
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SHRX EAX, [0x2000], ECX
        0xf4,
    ];
    emu.regs_mut().rcx = 4;
    emu.load_code_bytes(&code);
    use crate::*;
    emu.maps.write_dword(DATA_ADDR, 0x12340000);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0x01234000,
        "Memory operand shifted correctly"
    );
}

#[test]
fn test_shrx_64bit_memory_operand() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHRX 64-bit with memory source
    let code = [
        0xc4, 0xe2, 0xf3, 0xf7, 0x04, 0x25, 0x00, 0x20, 0x00, 0x00, // SHRX RAX, [0x2000], RCX
        0xf4,
    ];
    emu.regs_mut().rcx = 8;
    emu.load_code_bytes(&code);
    use crate::*;
    emu.maps.write_qword(DATA_ADDR, 0x1234567800000000);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax, 0x0012345678000000, "64-bit memory operand shifted");
}

// ============================================================================
// Extended Register Tests (R8-R15)
// ============================================================================

#[test]
fn test_shrx_32bit_r8d_r9d_r10d() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHRX R8D, R9D, R10D
    let code = [
        0xc4, 0x42, 0x2b, 0xf7, 0xc1, // SHRX R8D, R9D, R10D
        0xf4,
    ];
    emu.regs_mut().r9 = 0x80000000;
    emu.regs_mut().r10 = 16;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().r8 & 0xFFFFFFFF,
        0x00008000,
        "Extended registers work correctly"
    );
}

#[test]
fn test_shrx_64bit_r14_r15_r13() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    // SHRX R14, R15, R13
    let code = [
        0xc4, 0x42, 0x93, 0xf7, 0xf7, // SHRX R14, R15, R13
        0xf4,
    ];
    emu.regs_mut().r15 = 0x8000000000000000;
    emu.regs_mut().r13 = 48;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().r14, 0x0000000000008000, "64-bit extended registers");
}

// ============================================================================
// Edge Cases
// ============================================================================

#[test]
fn test_shrx_32bit_shift_all_bits_out() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x000000FF;
    emu.regs_mut().rcx = 8;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0, "All bits shifted out");
}

#[test]
fn test_shrx_source_preservation() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x12345678;
    emu.regs_mut().rcx = 8;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rbx & 0xFFFFFFFF, 0x12345678, "Source unchanged");
    assert_eq!(emu.regs().rcx & 0xFFFFFFFF, 8, "Count unchanged");
}

#[test]
fn test_shrx_consecutive_shifts() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0x48, 0x89, 0xc3,             // MOV RBX, RAX
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0x01000000;
    emu.regs_mut().rcx = 4;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFFFFFFFF, 0x00010000, "Consecutive shifts accumulate");
}

#[test]
fn test_shrx_comparison_with_sarx() {
    let DATA_ADDR = 0x7000;
    let mut emu = emu64();
    let code = [
        0xc4, 0xe2, 0x73, 0xf7, 0xc3, // SHRX EAX, EBX, ECX
        0xf4,
    ];
    emu.regs_mut().rbx = 0xF0000000; // Negative in signed interpretation
    emu.regs_mut().rcx = 4;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(
        emu.regs().rax & 0xFFFFFFFF,
        0x0F000000,
        "SHRX should zero-extend, not sign-extend"
    );
}
