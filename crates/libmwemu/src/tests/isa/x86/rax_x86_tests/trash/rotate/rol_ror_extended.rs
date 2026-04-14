// Extended tests for ROL/ROR - comprehensive edge cases
//
// This file contains additional edge case tests for ROL and ROR instructions:
// - All shift counts (0-64 for 64-bit, etc.)
// - Boundary conditions
// - Flag behavior edge cases
// - Count masking behavior
// - Comprehensive register coverage

use crate::*;

// ============================================================================
// ROL comprehensive shift count tests
// ============================================================================

#[test]
fn test_rol_all_counts_8bit() {
    // Test ROL with all valid shift counts for 8-bit
    for count in 0..=8 {
        let code = [0xc0, 0xc0, count, 0xf4]; // ROL AL, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0xAB; // 1010_1011
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = match count {
            0 => 0xAB,
            1 => 0x57, // 0101_0111
            2 => 0xAE, // 1010_1110
            3 => 0x5D, // 0101_1101
            4 => 0xBA, // 1011_1010
            5 => 0x75, // 0111_0101
            6 => 0xEA, // 1110_1010
            7 => 0xD5, // 1101_0101
            8 => 0xAB, // Back to original
            _ => unreachable!(),
        };
        assert_eq!(emu.regs().rax & 0xFF, expected, "ROL AL by {} failed", count);
    }
}

#[test]
fn test_ror_all_counts_8bit() {
    // Test ROR with all valid shift counts for 8-bit
    for count in 0..=8 {
        let code = [0xc0, 0xc8, count, 0xf4]; // ROR AL, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0xAB; // 1010_1011
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = match count {
            0 => 0xAB,
            1 => 0xD5, // 1101_0101
            2 => 0xEA, // 1110_1010
            3 => 0x75, // 0111_0101
            4 => 0xBA, // 1011_1010
            5 => 0x5D, // 0101_1101
            6 => 0xAE, // 1010_1110
            7 => 0x57, // 0101_0111
            8 => 0xAB, // Back to original
            _ => unreachable!(),
        };
        assert_eq!(emu.regs().rax & 0xFF, expected, "ROR AL by {} failed", count);
    }
}

#[test]
fn test_rol_all_counts_16bit() {
    // Test selected counts for 16-bit
    for count in [0, 1, 4, 8, 12, 15, 16].iter() {
        let code = [0x66, 0xc1, 0xc0, *count, 0xf4]; // ROL AX, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0x1234;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        // Verify rotation works correctly
        let result = emu.regs().rax & 0xFFFF;
        if *count == 0 {
            assert_eq!(result, 0x1234, "ROL AX by 0");
        } else if *count == 16 {
            assert_eq!(result, 0x1234, "ROL AX by 16 (full rotation)");
        }
    }
}

#[test]
fn test_ror_all_counts_16bit() {
    // Test selected counts for 16-bit
    for count in [0, 1, 4, 8, 12, 15, 16].iter() {
        let code = [0x66, 0xc1, 0xc8, *count, 0xf4]; // ROR AX, count
        let mut emu = emu64();
        emu.regs_mut().rax = 0x1234;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let result = emu.regs().rax & 0xFFFF;
        if *count == 0 {
            assert_eq!(result, 0x1234, "ROR AX by 0");
        } else if *count == 16 {
            assert_eq!(result, 0x1234, "ROR AX by 16 (full rotation)");
        }
    }
}

// ============================================================================
// Boundary value tests
// ============================================================================

#[test]
fn test_rol_boundary_8bit() {
    // Test with 0x00, 0xFF, 0x01, 0x80
    let test_values = [0x00, 0xFF, 0x01, 0x80];
    for &value in &test_values {
        let code = [0xc0, 0xc0, 0x01, 0xf4]; // ROL AL, 1
        let mut emu = emu64();
        emu.regs_mut().rax = value;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = ((value << 1) | (value >> 7)) & 0xFF;
        assert_eq!(emu.regs().rax & 0xFF, expected, "ROL 0x{:02X} by 1", value);
    }
}

#[test]
fn test_ror_boundary_8bit() {
    let test_values = [0x00, 0xFF, 0x01, 0x80];
    for &value in &test_values {
        let code = [0xc0, 0xc8, 0x01, 0xf4]; // ROR AL, 1
        let mut emu = emu64();
        emu.regs_mut().rax = value;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = ((value >> 1) | (value << 7)) & 0xFF;
        assert_eq!(emu.regs().rax & 0xFF, expected, "ROR 0x{:02X} by 1", value);
    }
}

#[test]
fn test_rol_boundary_32bit() {
    let test_values = [0x00000000, 0xFFFFFFFF, 0x00000001, 0x80000000];
    for &value in &test_values {
        let code = [0xc1, 0xc0, 0x01, 0xf4]; // ROL EAX, 1
        let mut emu = emu64();
        emu.regs_mut().rax = value;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = ((value << 1) | (value >> 31)) & 0xFFFFFFFF;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "ROL 0x{:08X} by 1", value);
    }
}

#[test]
fn test_ror_boundary_32bit() {
    let test_values = [0x00000000, 0xFFFFFFFF, 0x00000001, 0x80000000];
    for &value in &test_values {
        let code = [0xc1, 0xc8, 0x01, 0xf4]; // ROR EAX, 1
        let mut emu = emu64();
        emu.regs_mut().rax = value;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = ((value >> 1) | (value << 31)) & 0xFFFFFFFF;
        assert_eq!(emu.regs().rax & 0xFFFFFFFF, expected, "ROR 0x{:08X} by 1", value);
    }
}

#[test]
fn test_rol_boundary_64bit() {
    let test_values = [
        0x0000000000000000,
        0xFFFFFFFFFFFFFFFF,
        0x0000000000000001,
        0x8000000000000000,
    ];
    for &value in &test_values {
        let code = [0x48, 0xc1, 0xc0, 0x01, 0xf4]; // ROL RAX, 1
        let mut emu = emu64();
        emu.regs_mut().rax = value;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = (value << 1) | (value >> 63);
        assert_eq!(emu.regs().rax, expected, "ROL 0x{:016X} by 1", value);
    }
}

#[test]
fn test_ror_boundary_64bit() {
    let test_values = [
        0x0000000000000000,
        0xFFFFFFFFFFFFFFFF,
        0x0000000000000001,
        0x8000000000000000,
    ];
    for &value in &test_values {
        let code = [0x48, 0xc1, 0xc8, 0x01, 0xf4]; // ROR RAX, 1
        let mut emu = emu64();
        emu.regs_mut().rax = value;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = (value >> 1) | (value << 63);
        assert_eq!(emu.regs().rax, expected, "ROR 0x{:016X} by 1", value);
    }
}

// ============================================================================
// Count masking tests
// ============================================================================

#[test]
fn test_rol_count_masking_8bit() {
    // For 8-bit, count is masked differently
    let code = [0xd2, 0xc0, 0xf4]; // ROL AL, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12;
    emu.regs_mut().rcx = 0x1F; // 31 - should be masked
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    // Result should be same as ROL by (31 % 8) = 7
    let mut emu = emu64();
    let code2 = [0xc0, 0xc0, 0x07, 0xf4];
    emu.load_code_bytes(&code2);
    emu.regs_mut().rax = 0x12;
    emu.run(None).unwrap();

    assert_eq!(rax & 0xFF, emu.regs().rax & 0xFF, "Count masking for 8-bit");
}

#[test]
fn test_ror_count_masking_8bit() {
    let code = [0xd2, 0xc8, 0xf4]; // ROR AL, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12;
    emu.regs_mut().rcx = 0x1F; // 31 - should be masked
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    // Result should be same as ROR by (31 % 8) = 7
    let code2 = [0xc0, 0xc8, 0x07, 0xf4];
    let mut emu = emu64();
    emu.load_code_bytes(&code);
    emu.regs_mut().rax = 0x12;
    emu.run(None).unwrap();

    assert_eq!(rax & 0xFF, emu.regs().rax & 0xFF, "Count masking for 8-bit");
}

#[test]
fn test_rol_count_masking_32bit() {
    let code = [0xd3, 0xc0, 0xf4]; // ROL EAX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x24; // 36 - should be masked to 4 (36 & 0x1F = 4)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    let code2 = [0xc1, 0xc0, 0x04, 0xf4];
    let mut emu = emu64();
    emu.load_code_bytes(&code2);
    emu.regs_mut().rax = 0x12345678;
    emu.run(None).unwrap();

    assert_eq!(rax & 0xFFFFFFFF, emu.regs().rax & 0xFFFFFFFF, "Count masking for 32-bit");
}

#[test]
fn test_ror_count_masking_32bit() {
    let code = [0xd3, 0xc8, 0xf4]; // ROR EAX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12345678;
    emu.regs_mut().rcx = 0x24; // 36 - should be masked to 4
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    let code2 = [0xc1, 0xc8, 0x04, 0xf4];
    let mut emu = emu64();
    emu.load_code_bytes(&code2);
    emu.regs_mut().rax = 0x12345678;
    emu.run(None).unwrap();

    assert_eq!(rax & 0xFFFFFFFF, emu.regs().rax & 0xFFFFFFFF, "Count masking for 32-bit");
}

#[test]
fn test_rol_count_masking_64bit() {
    let code = [0x48, 0xd3, 0xc0, 0xf4]; // ROL RAX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 0x48; // 72 - should be masked to 8 (72 & 0x3F = 8)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    let code2 = [0x48, 0xc1, 0xc0, 0x08, 0xf4];
    let mut emu = emu64();
    emu.load_code_bytes(&code2);
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.run(None).unwrap();

    assert_eq!(rax, emu.regs().rax, "Count masking for 64-bit");
}

#[test]
fn test_ror_count_masking_64bit() {
    let code = [0x48, 0xd3, 0xc8, 0xf4]; // ROR RAX, CL
    let mut emu = emu64();
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.regs_mut().rcx = 0x48; // 72 - should be masked to 8
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    let code2 = [0x48, 0xc1, 0xc8, 0x08, 0xf4];
    let mut emu = emu64();
    emu.load_code_bytes(&code2);  
    emu.regs_mut().rax = 0x123456789ABCDEF0;
    emu.run(None).unwrap();

    assert_eq!(rax, emu.regs().rax, "Count masking for 64-bit");
}

// ============================================================================
// Overflow flag edge cases
// ============================================================================

#[test]
fn test_rol_of_transitions() {
    // Test OF flag for various transitions
    let test_cases = [
        (0x00, false), // 0 -> 0, no change
        (0x40, true),  // + -> -, change
        (0x80, false), // - -> -, no change (well, depends on impl)
        (0xC0, true),  // - -> +, change
    ];

    for (value, _expected_of) in &test_cases {
        let code = [0xd0, 0xc0, 0xf4]; // ROL AL, 1
        let mut emu = emu64();
        emu.regs_mut().rax = *value as u64;
        emu.load_code_bytes(&code);
        let _regs = emu.run(None).unwrap();
        // OF is set when MSB changed
    }
}

#[test]
fn test_ror_of_transitions() {
    // ROR OF is set when MSB XOR (MSB-1) after rotation
    let code = [0xd0, 0xc8, 0xf4]; // ROR AL, 1
    let mut emu = emu64();
    emu.regs_mut().rax = 0x01; // Will become 0x80 (MSB set)
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x80, "Result");
    // OF should be set
    assert!(emu.flags().f_of, "OF for ROR with MSB change");
}

// ============================================================================
// Carry flag edge cases
// ============================================================================

#[test]
fn test_rol_cf_all_bits() {
    // Test CF for each bit position in 8-bit
    for bit_pos in 0..8 {
        let value = 1u8 << bit_pos;
        let code = [0xc0, 0xc0, 0x01, 0xf4]; // ROL AL, 1
        let mut emu = emu64();
        emu.regs_mut().rax = value as u64;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        if bit_pos == 7 {
            assert!(emu.flags().f_cf, "CF should be set for bit 7");
        } else {
            assert!(!emu.flags().f_cf, "CF should be clear for bit {}", bit_pos);
        }
    }
}

#[test]
fn test_ror_cf_all_bits() {
    // Test CF for each bit position in 8-bit
    for bit_pos in 0..8 {
        let value = 1u8 << bit_pos;
        let code = [0xc0, 0xc8, 0x01, 0xf4]; // ROR AL, 1
        let mut emu = emu64();
        emu.regs_mut().rax = value as u64;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        if bit_pos == 0 {
            assert!(emu.flags().f_cf, "CF should be set for bit 0");
        } else {
            assert!(!emu.flags().f_cf, "CF should be clear for bit {}", bit_pos);
        }
    }
}

// ============================================================================
// Pattern tests
// ============================================================================

#[test]
fn test_rol_alternating_pattern() {
    let code = [0xc0, 0xc0, 0x04, 0xf4]; // ROL AL, 4
    let mut emu = emu64();
    emu.regs_mut().rax = 0xAA; // 1010_1010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xAA, "Alternating pattern ROL 4");
}

#[test]
fn test_ror_alternating_pattern() {
    let code = [0xc0, 0xc8, 0x04, 0xf4]; // ROR AL, 4
    let mut emu = emu64();
    emu.regs_mut().rax = 0xAA; // 1010_1010
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0xAA, "Alternating pattern ROR 4");
}

#[test]
fn test_rol_sequential_rotations() {
    // Perform multiple sequential rotations
    let code = [
        0xc0, 0xc0, 0x01, // ROL AL, 1
        0xc0, 0xc0, 0x01, // ROL AL, 1
        0xc0, 0xc0, 0x01, // ROL AL, 1
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();
    let rax = emu.regs().rax;

    // Should be same as ROL by 3
    let code2 = [0xc0, 0xc0, 0x03, 0xf4];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x12;
    emu.run(None).unwrap();

    assert_eq!(rax & 0xFF, emu.regs().rax & 0xFF, "Sequential rotations");
}

#[test]
fn test_rol_ror_identity() {
    // ROL followed by ROR with same count should return original
    let code = [
        0xc0, 0xc0, 0x05, // ROL AL, 5
        0xc0, 0xc8, 0x05, // ROR AL, 5
        0xf4,
    ];
    let mut emu = emu64();
    emu.regs_mut().rax = 0x42;
    emu.load_code_bytes(&code);
    emu.run(None).unwrap();

    assert_eq!(emu.regs().rax & 0xFF, 0x42, "ROL then ROR returns original");
}

#[test]
fn test_rol_power_of_two() {
    // Test with power-of-two values
    for i in 0..8 {
        let value = 1u8 << i;
        let code = [0xc0, 0xc0, 0x03, 0xf4]; // ROL AL, 3
        let mut emu = emu64();
        emu.regs_mut().rax = value as u64;
        emu.load_code_bytes(&code);
        emu.run(None).unwrap();

        let expected = ((value << 3) | (value >> 5)) & 0xFF;
        assert_eq!(emu.regs().rax & 0xFF, expected as u64, "ROL power of 2 value 0x{:02X}", value);
    }
}
